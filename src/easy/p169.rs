struct Solution;

use std::collections::HashMap;

impl Solution {
    // 使用 Hash
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut table = HashMap::with_capacity(nums.len());
        nums.into_iter().for_each(|num| {
            let cnt = table.entry(num).or_insert(0);
            *cnt += 1;
        });
        return *table
            .iter()
            .max_by(|(_, c1), (_, c2)| c1.cmp(c2))
            .unwrap()
            .0;
    }

    // 理论上可以使用堆排序找出第 nums.len() / 2 + 1 个元素, 这个元素就是答案
    pub fn majority_element2(mut nums: Vec<i32>) -> i32 {
        let k = nums.len() / 2;
        let (_, &mut res, _) = nums.select_nth_unstable(k);
        res
    }

    pub fn majority_element3(nums: Vec<i32>) -> i32 {
        let mut winner = nums[0];
        let mut cnt = 1;
        for &num in &nums[1..] {
            if cnt == 0 {
                winner = num;
                cnt = 1;
                continue;
            }
            if num == winner {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }
        winner
    }
}
