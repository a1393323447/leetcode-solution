use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut map = HashMap::with_capacity(k);
        for (i, n) in nums.into_iter().enumerate() {
            if let Some(pi) = map.get_mut(&n) {
                if i - *pi <= k {
                    return true;
                } else {
                    *pi = i;
                }
            } else {
                map.insert(n, i);
            }
        }

        false
    }
}
