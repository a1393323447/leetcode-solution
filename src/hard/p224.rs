use std::{iter::Peekable, str::Chars};

struct Solution;

macro_rules! next_token_str {
    ($lexer: expr, $step: expr) => {{
        let start = $lexer.pos;
        let end = $lexer.pos + $step;
        $lexer.pos = end;

        &$lexer.input[start..end]
    }};
}

#[derive(Debug, Clone, Copy)]
enum Token {
    /// (
    OpenParentheses,
    /// )
    CloseParentheses,
    /// `-`
    Dash,
    /// +
    Add,
    /// 1 2 3 ...
    Int(i32),
}

pub struct Lexer<'s> {
    input: &'s str,
    chars: Box<Peekable<Chars<'s>>>,
    pos: usize,
}

impl<'s> Lexer<'s> {
    /// Creates a new `Lexer`, given its source `input`.
    pub fn new(input: &'s str) -> Lexer<'s> {
        Lexer {
            input,
            chars: Box::new(input.chars().peekable()),
            pos: 0,
        }
    }

    pub fn lex(&mut self) -> Option<Token> {
        while let Some(ch) = self.chars.peek() {
            if ch.is_whitespace() {
                self.skip_white_space();
            } else if is_digit(ch) {
                return Some(self.lex_int());
            } else {
                return self.lex_symbol();
            }
        }

        None
    }

    /// A helper function for lex_num
    ///
    /// This function try to lex an int from source
    ///
    /// grammar:
    /// ```text
    /// positive = { '1'..'9' }
    /// digit = { '0'..'9' }
    /// int = { (positive ~ digit*) | '0' }
    /// ```
    fn lex_int(&mut self) -> Token {
        let mut step = 0;
        if let Some(ch) = self.chars.peek() {
            if *ch == '0' {
                step += 1;
                self.chars.next();
            } else if is_positive(ch) {
                step += 1;
                self.chars.next();
                while let Some(ch) = self.chars.peek() {
                    if is_digit(ch) {
                        step += 1;
                        self.chars.next();
                    } else {
                        break;
                    }
                }
            }
        }

        // because in function lex, this function would be called
        // only when is_digit(ch) == true
        debug_assert!(step > 0);

        let str = next_token_str!(self, step);

        Token::Int(str.parse::<i32>().unwrap())
    }

    /// lex a symbol from source
    ///
    /// grammar
    /// ```text
    /// symbol = {
    ///     '(' | ')' |
    ///     '+' | '-' |
    /// }
    /// ```
    fn lex_symbol(&mut self) -> Option<Token> {
        // a helper macro for step forword in chars
        if let Some(ch) = self.chars.peek() {
            if ch.is_ascii() {
                let ch = *ch;
                self.chars.next();
                self.pos += 1;

                if let Some(token) = SYMBOL_TABLE[ch as usize] {
                    Some(token)
                } else {
                    panic!("Unexpected symbol: {}", ch);
                }
            } else {
                panic!("unexpected symbol: {}", ch);
            }
        } else {
            None
        }
    }

    /// skip white space between token
    fn skip_white_space(&mut self) {
        while let Some(ch) = self.chars.peek() {
            if !ch.is_whitespace() {
                break;
            }

            self.pos += 1;
            self.chars.next();
        }
    }
}

const SYMBOL_TABLE: [Option<Token>; 128] = get_symbol_table();

const fn get_symbol_table() -> [Option<Token>; 128] {
    let mut table: [Option<Token>; 128] = [None; 128];

    table['(' as usize] = Some(Token::OpenParentheses);
    table[')' as usize] = Some(Token::CloseParentheses);

    table['+' as usize] = Some(Token::Add);
    table['-' as usize] = Some(Token::Dash);

    table
}

/// positive = { '1'..'9' }
fn is_positive(ch: &char) -> bool {
    ('1'..='9').contains(ch)
}

/// digit = { '0'..'9' }
fn is_digit(ch: &char) -> bool {
    ('0'..='9').contains(ch)
}

#[derive(Debug, Clone, Copy)]
enum UnaryOperator {
    Neg,
}

#[derive(Debug, Clone, Copy)]
enum BinOperator {
    Add,
    Sub,
}

enum Expr {
    Unary(Box<UnaryExpr>),
    Bin(Box<BinExpr>),
}

impl Expr {
    fn eval(&self) -> i32 {
        match self {
            Expr::Unary(e) => e.eval(),
            Expr::Bin(e) => e.eval(),
        }
    }
}

struct UnaryExpr {
    op: UnaryOperator,
    expr: Expr,
}

impl UnaryExpr {
    fn eval(&self) -> i32 {
        todo!()
    }
}

struct BinExpr {
    lhs: Expr,
    op: BinOperator,
    rhs: Expr,
}

impl BinExpr {
    fn eval(&self) -> i32 {
        todo!()
    }
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        todo!()
    }
}
