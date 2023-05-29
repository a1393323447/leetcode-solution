struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let chars = s.as_bytes();
        let table = Solution::generate_table(chars);

        let mut cache = vec![];
        let mut res = vec![];
        Solution::partition_impl(chars, 0, &mut cache, &mut res, &table);

        res
    }

    fn partition_impl(
        s: &[u8],
        start: usize,
        cache: &mut Vec<String>,
        res: &mut Vec<Vec<String>>,
        table: &Vec<Vec<bool>>,
    ) {
        if start == s.len() {
            res.push(cache.clone());
            return;
        }

        for i in start..s.len() {
            if table[start][i] {
                cache.push(unsafe { std::str::from_utf8_unchecked(&s[start..=i]).to_string() });
                Solution::partition_impl(s, i + 1, cache, res, table);
                cache.pop();
            }
        }
    }

    // generate a table T[i][j] which indicate whether s[i..=j] is a 回文串
    fn generate_table(s: &[u8]) -> Vec<Vec<bool>> {
        let len = s.len();
        let mut dp = vec![vec![false; len + 1]; len + 1];
        for i in 0..len {
            dp[i][i] = true;
        }
        // T[i][j] = T[i + 1][j - 1] && s[i] == s[j]
        for j in 1..len {
            for i in 0..j {
                if j - i < 3 {
                    dp[i][j] = s[i] == s[j];
                } else {
                    dp[i][j] = dp[i + 1][j - 1] && s[i] == s[j];
                }
            }
        }
        dp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dp() {
        let dp = Solution::generate_table(b"aabbc");
        for (i, flag) in dp.into_iter().enumerate() {
            for (j, f) in flag.into_iter().enumerate() {
                if f {
                    println!("({i}, {j})");
                }
            }
        }
    }
}
