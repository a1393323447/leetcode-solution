use std::collections::HashMap;

struct Solution;

impl Solution {
    // HashMap 记录字符第一次出现和最后一次出现的下标号
    pub fn first_uniq_char(s: String) -> i32 {
        let mut table = HashMap::new();
        for (idx, byte) in s.as_bytes().iter().enumerate() {
            table
                .entry(byte)
                .and_modify(|(_, i)| *i = idx)
                .or_insert((idx, idx));
        }

        table
            .into_values()
            .filter(|(f, l)| *f == *l)
            .min_by(|(f1, _), (f2, _)| f1.cmp(f2))
            .map(|(f, _)| f as i32)
            .unwrap_or(-1)
    }

    // 记录字符出现次数
    pub fn first_uniq_char2(s: String) -> i32 {
        let mut map = [0; 26];

        for ss in s.chars() {
            map[(ss as u8 - b'a') as usize] += 1;
        }

        for (idx, ss) in s.chars().enumerate() {
            if map[(ss as u8 - b'a') as usize] == 1 {
                return idx as i32;
            }
        }
        -1
    }
}
