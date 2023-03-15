use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = BTreeMap::new();
        dp.insert(1, nums[0]);
        let mut max_len = 1;
        for i in 1..n {
            let mut new_subarr = (1, nums[i]);
            for (&len, &min_val) in dp.iter().rev() {
                if nums[i] > min_val {
                    new_subarr.0 = len + 1;
                    max_len = max_len.max(new_subarr.0);
                    break;
                }
            }
            let min_val = dp.entry(new_subarr.0).or_insert(new_subarr.1);
            *min_val = new_subarr.1.min(*min_val);
        }

        max_len
    }
}
