struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let set: HashSet<_> = nums.iter().cloned().collect();
        nums.into_iter()
            .enumerate()
            .filter(|(_, num)| set.get(&(target - *num)).is_some())
            .map(|(idx, _)| idx as i32)
            .collect()
    }
}
