struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        let last_pos = nums.len() - 1;
        let mut max_idx = 0;
        for (idx, step) in nums.into_iter().enumerate() {
            max_idx = max_idx.max(idx + step as usize);
            if max_idx >= last_pos {
                return true;
            } else if step == 0 && max_idx <= idx {
                return false;
            }
        }

        true
    }
}
