struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        s.split('|')
            .enumerate()
            .filter(|&(idx, _)| idx % 2 == 1)
            .inspect(|&(_, s)| println!("{s}"))
            .map(|(_, s)| s.as_bytes().iter().filter(|&&b| b == b'*').count() as i32)
            .sum()
    }
}
