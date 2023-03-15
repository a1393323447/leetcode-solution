struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![-1; amount + 1];
        dp[0] = 0;

        for i in 1..dp.len() {
            let mut min = i32::MAX;
            for &coin in coins.iter() {
                let coin = coin as usize;
                if let Some(&cnt) = dp.get(i - coin) {
                    if cnt != -1 {
                        min = min.min(cnt + 1);
                    }
                }
            }
            if min != i32::MAX {
                dp[i] = min;
            } else {
                dp[i] = -1;
            }
        }

        dp[amount]
    }
}
