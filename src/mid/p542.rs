struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_size = mat.len();
        let col_size = mat[0].len();

        for y in 0..row_size {
            for x in 0..col_size {
                if mat[y][x] == 1 {
                    let dist = Solution::closest_dist(&mat, (y, x));
                    mat[y][x] = dist;
                }
            }
        }

        mat
    }

    fn closest_dist(mat: &Vec<Vec<i32>>, pos: (usize, usize)) -> i32 {
        let mut vis = vec![vec![false; mat[0].len()]; mat.len()];
        let mut queue = VecDeque::new();
        queue.push_back((pos.0, pos.1, 0));
        while let Some((y, x, dist)) = queue.pop_front() {
            if let Some(ele) = mat.get(y).map(|row| row.get(x)).flatten() {
                if *ele == 0 {
                    return dist;
                } else if *ele == 1 && !vis[y][x] {
                    vis[y][x] = true;
                    [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
                        .into_iter()
                        .for_each(|(y, x)| queue.push_back((y, x, dist + 1)));
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        let mat = vec![
            vec![0, 0, 1, 0, 1, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 0, 0, 0, 1, 1],
            vec![1, 0, 1, 0, 1, 1, 1, 0, 1, 1],
            vec![0, 0, 1, 1, 1, 0, 1, 1, 1, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0, 1, 0, 1, 0, 1],
            vec![0, 1, 0, 0, 0, 1, 0, 0, 1, 1],
            vec![1, 1, 1, 0, 1, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 0],
        ];

        Solution::update_matrix(mat);
    }
}
