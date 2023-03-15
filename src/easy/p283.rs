struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut idx = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[idx] = nums[i];
                idx += 1;
            }
        }

        nums[idx..].fill(0);
    }
}
