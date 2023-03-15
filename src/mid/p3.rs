use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut table = HashMap::new();
        let mut start = 0;
        let mut len = 0;
        let bytes = s.as_bytes();
        for (idx, &ch) in bytes.iter().enumerate() {
            if table.get(&ch).is_none() {
                table.insert(ch, idx);
            } else {
                let old_start = start;
                let start_idx = table.get_mut(&ch).unwrap();
                len = len.max(idx - start);
                start = *start_idx + 1;
                *start_idx = idx;

                // 删除其它的东西
                for byte in &bytes[old_start..(start - 1)] {
                    table.remove(byte);
                }
            }
            // 处理边界情况
            if idx == s.len() - 1 {
                len = len.max(idx - start + 1);
            }
        }

        len as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        Solution::length_of_longest_substring("abcabcbb".into());
    }
}
