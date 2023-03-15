#![allow(arithmetic_overflow)]

struct Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let bytes = word.as_bytes();
        let n = board.len();
        let m = board[0].len();
        for i in 0..n {
            for j in 0..m {
                let byte = board[i][j] as u8;
                if byte == bytes[0] {
                    board[i][j] = (byte - b'A') as char;
                    let search_res = Solution::search(&mut board, &bytes[1..], (i, j));
                    board[i][j] = (board[i][j] as u8 + b'A') as char;
                    if search_res {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn search(board: &mut Vec<Vec<char>>, word: &[u8], pos: (usize, usize)) -> bool {
        if word.is_empty() {
            return true;
        }

        let try_get: for<'a> fn(&'a mut Vec<Vec<char>>, (usize, usize)) -> Option<&'a mut char> =
            |board: &mut Vec<Vec<char>>, pos: (usize, usize)| {
                board.get_mut(pos.0).map(|row| row.get_mut(pos.1)).flatten()
            };

        let mut try_search = |pos: (usize, usize)| {
            let mut search_res = false;
            if let Some(ch) = try_get(board, pos) {
                if ch.is_ascii_alphabetic() && *ch as u8 == word[0] {
                    *ch = (*ch as u8 - b'A') as char;
                    search_res = Solution::search(board, &word[1..], pos);
                    let ch = try_get(board, pos).unwrap();
                    *ch = (*ch as u8 + b'A') as char;
                }
            }

            search_res
        };

        try_search((pos.0 - 1, pos.1))
            || try_search((pos.0 + 1, pos.1))
            || try_search((pos.0, pos.1 - 1))
            || try_search((pos.0, pos.1 + 1))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        Solution::exist(
            vec![
                vec!['C', 'A', 'A'],
                vec!['A', 'A', 'A'],
                vec!['B', 'C', 'D'],
            ],
            "AAB".into(),
        );
    }
}
