struct Solution;

impl Solution {
    fn two_sum(nums: &[i32], res: &mut i32, diff: &mut i32, value: i32, target: i32) {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let n1 = nums[left];
            let n2 = nums[right];
            let sum = n1 + n2;

            let this_diff = target.saturating_sub(sum).abs();
            if this_diff < *diff {
                *diff = this_diff;
                *res = sum + value;
            }

            match sum.cmp(&target) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
                std::cmp::Ordering::Equal => return,
            }
        }
    }

    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();

        let mut res = i32::MAX;
        let mut diff = i32::MAX;
        for i in 0..(nums.len() - 2) {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }
            let value = nums[i];
            Solution::two_sum(&nums[(i + 1)..], &mut res, &mut diff, value, target - value)
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
        assert_eq!(
            Solution::three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2),
            -2
        );
    }
}
