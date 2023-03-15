struct Solution;

impl Solution {
    fn two_sum(nums: &[i32], res: &mut Vec<Vec<i32>>, value: i32, target: i32) {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let n1 = nums[left];
            let n2 = nums[right];
            let sum = n1 + n2;
            match sum.cmp(&target) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Equal => {
                    res.push(vec![value, n1, n2]);
                    // 去重
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    // 移动指针
                    left += 1;
                    right -= 1;
                }
                std::cmp::Ordering::Greater => right -= 1,
            }
        }
    }

    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        nums.sort();

        let mut res = vec![];
        for i in 0..(nums.len() - 2) {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }
            let value = nums[i];
            Solution::two_sum(&nums[(i + 1)..], &mut res, value, -value)
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
}
