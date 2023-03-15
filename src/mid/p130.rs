use std::collections::VecDeque;

struct Solution;

const UNBOUND: char = '*';

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let n = board.len();
        let m = board[0].len();

        for ni in 0..n {
            if board[ni][0] == 'O' {
                Solution::fill(board, (ni, 0));
            }
            if board[ni][m - 1] == 'O' {
                Solution::fill(board, (ni, m - 1));
            }
        }

        for mi in 0..m {
            if board[0][mi] == 'O' {
                Solution::fill(board, (0, mi));
            }
            if board[n - 1][mi] == 'O' {
                Solution::fill(board, (n - 1, mi));
            }
        }

        for v in board {
            for ch in v {
                if *ch == UNBOUND {
                    *ch = 'O';
                } else if *ch == 'O' {
                    *ch = 'X';
                }
            }
        }
    }

    fn fill(board: &mut Vec<Vec<char>>, (ns, ms): (usize, usize)) {
        let n = board.len();
        let m = board[0].len();

        let mut vis = vec![vec![false; m]; n];
        let mut queue = VecDeque::new();
        queue.push_back((ns, ms));

        while let Some((ni, mi)) = queue.pop_front() {
            if let Some(ch) = board.get_mut(ni).map(|v| v.get_mut(mi)).flatten() {
                if !vis[ni][mi] && *ch == 'O' {
                    vis[ni][mi] = true;
                    *ch = UNBOUND;
                    queue.extend(
                        [(ni - 1, mi), (ni + 1, mi), (ni, mi - 1), (ni, mi + 1)].into_iter(),
                    );
                }
            }
        }
    }
}
