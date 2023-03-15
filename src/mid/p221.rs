struct Solution;

impl Solution {
    // 暴力 dp
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let rn = matrix.len();
        let cn = matrix[0].len();
        let ml = rn.max(cn);

        let mut max_area = 0;
        let mut dp = vec![vec![vec![false; cn]; rn]; ml + 1];

        for ri in 0..rn {
            for ci in 0..cn {
                if matrix[ri][ci] == '1' {
                    dp[1][ri][ci] = true;
                    max_area = 1;
                }
            }
        }

        for len in 2..=ml {
            let r = rn.saturating_sub(len - 1);
            let c = cn.saturating_sub(len - 1);
            for ri in 0..r {
                for ci in 0..c {
                    if dp[len - 1][ri][ci] && Solution::check(&matrix, (ri, ci), len) {
                        max_area = max_area.max(len * len);
                        dp[len][ri][ci] = true;
                    }
                }
            }
        }

        max_area as i32
    }

    // 暴力遍历
    fn maximal_square2(matrix: Vec<Vec<char>>) -> i32 {
        let rn = matrix.len();
        let cn = matrix[0].len();

        let mut max_area = 0;

        for ri in 0..rn {
            for ci in 0..cn {
                let l = (rn - ri).min(cn - ci) + 1;
                for len in 1..l {
                    if Solution::check(&matrix, (ri, ci), len) {
                        max_area = max_area.max(len * len);
                    } else {
                        break;
                    }
                }
            }
        }

        max_area as i32
    }

    fn check(matrix: &Vec<Vec<char>>, (r, c): (usize, usize), len: usize) -> bool {
        if matrix[r + len - 1][c..c + len].iter().any(|ch| *ch == '0') {
            return false;
        }

        for ri in r..r + len {
            if matrix[ri][c + len - 1] == '0' {
                return false;
            }
        }

        true
    }
}

// 二分
impl Solution {
    pub fn maximal_square3(matrix: Vec<Vec<char>>) -> i32 {
        let rn = matrix.len();
        let cn = matrix[0].len();
        let ml = rn.max(cn);

        let mut left = 1;
        let mut right = ml;
        let mut ans = 0;

        while left <= right {
            let mid = (left + right) / 2;
            let r = rn.saturating_sub(mid - 1);
            let c = cn.saturating_sub(mid - 1);
            let mut flag = false;

            for ri in 0..r {
                for ci in 0..c {
                    if Solution::check3(&matrix, (ri, ci), mid) {
                        flag = true;
                        break;
                    }
                }
            }

            if flag {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        (ans * ans) as i32
    }

    fn check3(matrix: &Vec<Vec<char>>, (r, c): (usize, usize), len: usize) -> bool {
        matrix[r..r + len]
            .iter()
            .map(|v| v[c..c + len].iter())
            .flatten()
            .all(|ch| *ch == '1')
    }
}

impl Solution {
    // dp[i][j] 表示以 (i, j) 为右下角的最大正方形边长
    // dp[i][j] = min(dp[i - 1][j], dp[i][j - 1], dp[i - 1][j - 1]) + 1
    pub fn maximal_square4(matrix: Vec<Vec<char>>) -> i32 {
        let rn = matrix.len();
        let cn = matrix[0].len();

        let mut dp: Vec<Vec<_>> = matrix
            .iter()
            .map(|v| {
                v.clone()
                    .into_iter()
                    .map(|ch| (ch as u8 - b'0') as i32)
                    .collect()
            })
            .collect();

        let mut max_len = -1;
        for ri in 1..rn {
            for ci in 1..cn {
                if matrix[ri][ci] != '1' {
                    continue;
                }

                let cur_len = dp[ri - 1][ci].min(dp[ri][ci - 1]).min(dp[ri - 1][ci - 1]) + 1;
                dp[ri][ci] = cur_len;
                max_len = max_len.max(cur_len);
            }
        }

        if max_len == -1 {
            max_len = 0;

            if matrix[0].iter().any(|ch| *ch == '1') {
                max_len = 1;
            }

            for ri in 0..rn {
                if matrix[ri][0] == '1' {
                    max_len = 1;
                    break;
                }
            }
        }

        max_len * max_len
    }
}
