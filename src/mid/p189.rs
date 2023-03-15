struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = nums.len() - k as usize % nums.len();
        nums[..k].reverse();
        nums[k..].reverse();
        nums.reverse();
    }
}
