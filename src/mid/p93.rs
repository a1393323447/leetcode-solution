struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let mut cache = String::with_capacity(s.len() + 4);
        let mut res = vec![];

        Solution::search(s, 0, &mut cache, &mut res);

        res
    }

    fn search(s: &[u8], dot_cnt: usize, cache: &mut String, res: &mut Vec<String>) {
        if s.is_empty() {
            if dot_cnt == 4 {
                cache.pop();
                res.push(cache.clone());
                cache.push('.');
            }
            return;
        }

        if dot_cnt == 4 {
            return;
        }

        let slen = s.len();
        for len in 1..=3 {
            if len > slen {
                break;
            }
            let slice = &s[..len];
            if Solution::check(slice) {
                for &b in slice {
                    cache.push(b as char);
                }
                cache.push('.');
                Solution::search(&s[len..], dot_cnt + 1, cache, res);
                cache.pop();
                for _ in 0..slice.len() {
                    cache.pop();
                }
            }
        }
    }

    fn check(bytes: &[u8]) -> bool {
        if bytes.len() == 1 {
            return bytes[0] <= b'9';
        }

        let mut has_num = false;
        let mut num = 0;
        for &byte in bytes {
            if byte == b'0' && !has_num {
                // 前导零
                return false;
            }
            let byte = (byte - b'0') as isize;
            num = num * 10 + byte;
            has_num = true;
        }

        return num <= 255;
    }
}
