struct Solution;

impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        nums.iter_mut().for_each(|n| *n *= *n);
        nums.sort();

        nums
    }
}
