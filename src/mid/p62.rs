struct Solution;

fn count_combinations(n: usize, r: usize) -> usize {
    if r > n {
        0
    } else {
        (1..=r.min(n - r)).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        count_combinations((m + n - 2) as usize, (n - 1) as usize) as i32
    }
}
