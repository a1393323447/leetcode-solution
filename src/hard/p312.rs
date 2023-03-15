struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let exnums = {
            let mut exnums = vec![0; len + 2];
            exnums[0] = 1;
            exnums[len + 1] = 1;
            for i in 1..(len + 1) {
                exnums[i] = nums[i - 1];
            }
            exnums
        };

        // dp[l][r] => 戳破 (l, r) 的气球可得的最大硬币数
        let mut dp = vec![vec![0; len + 2]; len + 2];
        for l in (0..len + 2).rev() {
            for r in l..len + 2 {
                for idx in (l + 1)..r {
                    // O O O O O O X O O O O O O
                    // l x x x x x i x x x x x r
                    //       戳          戳
                    // l  i  r
                    //    戳
                    let val = dp[l][idx] + exnums[l] * exnums[idx] * exnums[r] + dp[idx][r];
                    dp[l][r] = dp[l][r].max(val);
                }
            }
        }

        dp[0][nums.len() + 1]
    }
}
