struct Solution;

impl Solution {
    // dp 可以被优化掉 => 和爬楼梯一样
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();

        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;
        if s[0] != b'0' {
            dp[1] = 1;
        }

        for i in 2..dp.len() {
            if s[i - 1] != b'0' {
                dp[i] = dp[i - 1]
            }
            if Solution::check(&s[(i - 2)..i]) {
                dp[i] += dp[i - 2];
            }
        }

        dp[s.len()]
    }

    fn check(s: &[u8]) -> bool {
        (s[0] == b'1' && (b'0'..=b'9').contains(&s[1]))
            || (s[0] == b'2' && (b'0'..=b'6').contains(&s[1]))
    }
}
