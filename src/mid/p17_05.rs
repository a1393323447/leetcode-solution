use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
        let mut info = HashMap::with_capacity(array.len());
        info.insert(0, 0);

        let mut max_len = 0;
        let mut start = array.len() - 1;

        let mut infoi = 0;
        for i in 0..array.len() {
            let s = array[i].as_bytes();
            if s[0].is_ascii_alphabetic() {
                infoi += 1;
            } else {
                infoi -= 1;
            }

            if let Some(find_pos) = info.get_mut(&infoi) {
                let cur_len = i - *find_pos + 1;
                let cur_start = *find_pos;
                if max_len < cur_len {
                    max_len = cur_len;
                    start = cur_start;
                }
            } else {
                info.insert(infoi, i + 1);
            }
        }

        array[start..start + max_len].iter().cloned().collect()
    }
}
