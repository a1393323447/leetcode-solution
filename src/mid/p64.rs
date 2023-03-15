struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; n]; m];

        dp[0][0] = grid[0][0];
        for i in 1..n {
            dp[0][i] = grid[0][i] + dp[0][i - 1];
        }

        for i in 1..m {
            dp[i][0] = grid[i][0] + dp[i - 1][0];
        }

        for i in 1..m {
            for j in 1..n {
                let cur = grid[i][j];
                let top = dp[i - 1][j] + cur;
                let left = dp[i][j - 1] + cur;
                dp[i][j] = top.min(left);
            }
        }

        dp[m - 1][n - 1]
    }
}
