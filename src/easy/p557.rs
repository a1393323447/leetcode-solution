struct Solution;

impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        s.push(' ');

        let bytes = unsafe { s.as_bytes_mut() };
        let byte_len = bytes.len();

        let mut fast = 0;
        let mut slow = 0;

        while fast < byte_len {
            if bytes[fast].is_ascii_graphic() {
                fast += 1;
                continue;
            } else {
                // 反转字符
                bytes[slow..fast].reverse();
                // 跳过空格
                fast += 1;
                slow = fast;
            }
        }

        s.pop();

        s
    }

    // 非原地
    pub fn reverse_words2(s: String) -> String {
        s.split_ascii_whitespace()
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
