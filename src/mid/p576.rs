use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1_len = s1.len();
        let mut table = HashMap::new();
        for byte in s1.into_bytes() {
            table.entry(byte).and_modify(|cnt| *cnt += 1).or_insert(1);
        }

        let bytes = s2.into_bytes();
        let size = bytes.len();
        let mut start = 0;
        let mut end = 0;
        let mut len = 0;
        while end < size {
            let cur_byte = bytes[end];
            match table.get_mut(&cur_byte) {
                Some(cnt) if *cnt <= 0 => {
                    while bytes[start] != cur_byte {
                        table.entry(bytes[start]).and_modify(|cnt| *cnt += 1);
                        start += 1;
                        len -= 1;
                    }
                    start += 1;
                    end += 1;
                }
                Some(cnt) => {
                    // cnt > 0
                    *cnt -= 1;
                    end += 1;
                    len += 1;
                }
                None => {
                    len = 0;
                    for &byte in &bytes[start..end] {
                        table.entry(byte).and_modify(|cnt| *cnt += 1);
                    }
                    start = end + 1;
                    end = start;
                }
            }
            if len == s1_len {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        Solution::check_inclusion("ky".into(), "ainwkckifykxlribaypk".into());
    }
}
