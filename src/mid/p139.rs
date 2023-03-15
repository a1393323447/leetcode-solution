use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s = s.as_bytes();
        let table: HashSet<_> = word_dict.into_iter().map(|s| s.into_bytes()).collect();
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 1..dp.len() {
            let mut dpi = i - 1;
            loop {
                if dp[dpi] && table.contains(&s[dpi..i]) {
                    dp[i] = true;
                    break;
                }
                if dpi == 0 {
                    break;
                }
                dpi -= 1;
            }
        }

        dp[s.len()]
    }
}
