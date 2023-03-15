struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (1..=n).fold((0, 1), |sum, _| (sum.1, sum.0 + sum.1)).1
    }
}
