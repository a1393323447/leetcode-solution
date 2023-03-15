use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let len = nums.len();
        let mut res = nums[0];
        let mut dp = BinaryHeap::new();

        dp.push((nums[0], 0));

        for idx in 1..len {
            // 为什么用 pop
            // 1. i < idx 且 idx 递增
            // 2. if idx - i > k => idx + 1 - i > k => 以后再也用不到这个
            while let Some(&(_, i)) = dp.peek() {
                if idx - i <= k {
                    break;
                }
                dp.pop();
            }

            res = dp.peek().unwrap().0 + nums[idx];
            dp.push((res, idx));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let res = Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2);
        println!("{res}");
    }
}
