struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        if s1.is_empty() && s2.is_empty() {
            return s3.is_empty();
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();

        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];

        dp[0][0] = true;

        for p1 in 1..=s1.len() {
            dp[p1][0] = s1[p1 - 1] == s3[p1 - 1] && dp[p1 - 1][0];
        }

        for p2 in 1..=s2.len() {
            dp[0][p2] = s2[p2 - 1] == s3[p2 - 1] && dp[0][p2 - 1];
        }

        for p1 in 1..=s1.len() {
            for p2 in 1..=s2.len() {
                dp[p1][p2] = (dp[p1 - 1][p2] && s1[p1 - 1] == s3[p1 + p2 - 1])
                    || (dp[p1][p2 - 1] && s2[p2 - 1] == s3[p1 + p2 - 1]);
            }
        }

        dp[s1.len()][s2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let s1 = "aabcc".into();
        let s2 = "dbbca".into();
        let s3 = "aadbbcbcac".into();

        Solution::is_interleave(s1, s2, s3);
    }
}
