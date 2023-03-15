struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.len() == 1 {
            return nums[0] == target;
        }
        // 找到 k
        let mut k = 1;
        while k < nums.len() && nums[k] >= nums[k - 1] {
            k += 1;
        }

        if nums[..k].binary_search(&target).is_ok() {
            return true;
        }

        if nums[k..].binary_search(&target).is_ok() {
            return true;
        }

        false
    }
}
