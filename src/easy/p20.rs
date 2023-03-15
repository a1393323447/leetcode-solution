struct Solution;

impl Solution {
    fn is_match(ch1: char, ch2: char) -> bool {
        matches!((ch1, ch2), ('(', ')') | ('{', '}') | ('[', ']'))
    }

    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for ch in s.chars() {
            match ch {
                '(' | '{' | '[' => stack.push(ch),
                ')' | '}' | ']' => match stack.pop() {
                    Some(c) if Solution::is_match(c, ch) => continue,
                    _ => return false,
                },
                _ => return false,
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::is_valid("()".into()), true);
    }
}
