struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        const BLOCK: i32 = 1;

        let n = obstacle_grid.len();
        let m = obstacle_grid[0].len();

        if obstacle_grid[n - 1][m - 1] == BLOCK {
            return 0;
        }

        let mut dp = vec![vec![0; m]; n];
        dp[0][0] = 1 - obstacle_grid[0][0];

        for ni in 1..n {
            if obstacle_grid[ni - 1][0] != BLOCK {
                dp[ni][0] = 1;
            } else {
                break;
            }
        }

        for mi in 1..m {
            if obstacle_grid[0][mi - 1] != BLOCK {
                dp[0][mi] = 1;
            } else {
                break;
            }
        }

        for ni in 1..n {
            for mi in 1..m {
                if obstacle_grid[ni - 1][mi] != BLOCK {
                    dp[ni][mi] += dp[ni - 1][mi];
                }
                if obstacle_grid[ni][mi - 1] != BLOCK {
                    dp[ni][mi] += dp[ni][mi - 1];
                }
            }
        }

        dp[n - 1][m - 1]
    }
}
