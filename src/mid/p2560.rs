use std::cmp::Ordering;

struct Solution;

impl Solution {
    // 最大值的最小、最小值的最大 => 单调性 => 二分

    // 总体思路 => 通过二分法猜一个盗窃能力, 检查在这个盗窃能力的限制下能偷到的房子数是否大于等于 k
    // 最后通过二分找到能满足条件的最小盗窃能力
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let max = *nums.iter().max().unwrap();
        let min = *nums.iter().min().unwrap();
        Solution::binary_search_lower(min, max, &nums, k)
    }

    fn binary_search_lower(min: i32, max: i32, nums: &[i32], k: i32) -> i32 {
        let mut left = min;
        let mut right = max;

        while left < right {
            let mid = (left + right) / 2;
            match Solution::check(nums, mid, k) {
                Ordering::Equal | Ordering::Greater => right = mid,
                Ordering::Less => left = mid + 1,
            }
        }

        left
    }

    fn check(nums: &[i32], max_ab: i32, k: i32) -> Ordering {
        // 先通过动态规划计算能够偷的房子数的最大值
        // dp[i] = max(dp[i - 1], dp[i - 2] + 1) if nums[i - 2] <= max_ab
        // dp[i] = dp[i - 1] if nums[i - 2] > max_ab

        // let mut dp = vec![0; nums.len() + 2];
        // for i in 0..nums.len() {
        //     if nums[i] > max_ab {
        //         dp[i + 2] = dp[i + 1];
        //     } else {
        //         dp[i + 2] = dp[i + 1].max(dp[i] + 1);
        //     }
        // }
        //
        // dp[nums.len() + 1].cmp(&k)

        // 滚动变量优化
        let mut dp0 = 0;
        let mut dp1 = 0;
        for i in 0..nums.len() {
            if nums[i] > max_ab {
                dp0 = dp1;
            } else {
                (dp1, dp0) = (dp1.max(dp0 + 1), dp1);
            }
        }

        dp1.cmp(&k)
    }
}
