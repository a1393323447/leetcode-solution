use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let p = p as i64;
        let len = nums.len();
        let r = nums.iter().map(|n| *n as i64).sum::<i64>() % p;

        if r == 0 {
            return 0;
        }

        let mut sum = 0;
        let mut map = HashMap::with_capacity(len + 1);

        let mut min_len = len;
        for (idx, num) in nums.into_iter().enumerate() {
            map.insert(sum, idx);

            sum = (sum + num as i64) % p;

            let expected = (sum - r + p) % p;
            if let Some(&find_idx) = map.get(&expected) {
                let cur_len = idx - find_idx + 1;
                min_len = min_len.min(cur_len);
            }
        }

        if min_len == len {
            return -1;
        }

        min_len as i32
    }
}
