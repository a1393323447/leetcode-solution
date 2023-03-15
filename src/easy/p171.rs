struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .as_bytes()
            .iter()
            .map(|byte| *byte - b'A' + 1)
            .fold(0, |acc, cur| acc * 26 + cur as i32)
    }
}
