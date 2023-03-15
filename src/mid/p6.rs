struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let mut res = vec![String::new(); num_rows];

        let mut cur_pos = 0;
        let mut delta: i32 = 1;
        for &ch in s.as_bytes() {
            res[cur_pos].push(ch as char);
            if cur_pos == 0 {
                delta = 1;
            } else if cur_pos == num_rows - 1 {
                delta = -1;
            } else {
                cur_pos += delta as usize;
            }
        }

        res.into_iter()
            .fold(String::new(), |acc, s| format!("{acc}{s}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        Solution::convert("ABCDEFGHIJKLNM".into(), 3);
    }
}
