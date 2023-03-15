struct Solution;

fn swap(nums: &mut [i32], i: usize, j: usize) {
    let temp = nums[i];
    nums[i] = nums[j];
    nums[j] = temp;
}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        const RED: i32 = 0;
        const WHITE: i32 = 1;
        const BLUE: i32 = 2;

        // 将红色和蓝色分开
        Solution::split(nums, RED, BLUE);
        // 将红色和白色分开
        Solution::split(nums, RED, WHITE);
        // 将蓝色和白色分开
        Solution::split(nums, WHITE, BLUE);
    }

    fn split(nums: &mut [i32], c1: i32, c2: i32) {
        let size = nums.len();
        let mut left = 0;
        let mut right = size - 1;
        while left < right {
            if nums[left] != c2 {
                left += 1;
                continue;
            }
            if nums[right] != c1 {
                right -= 1;
                continue;
            }
            swap(nums, left, right);

            left += 1;
            right -= 1;
        }
    }

    // 思路来自快排
    // 维护三个区间:
    // all in [0, p0) == 0
    // all in [p0, p1) == 1
    // all in (p2, l-1] == 2
    fn sort_colors2(nums: &mut Vec<i32>) {
        let mut p0 = 0;
        let mut p1 = 0;
        let mut p2 = nums.len() - 1;

        while (p1 as isize) <= (p2 as isize) {
            let num = nums[p1];
            if num == 0 {
                swap(nums, p0, p1);
                p0 += 1;
                p1 += 1;
            } else if num == 1 {
                p1 += 1;
            } else {
                // num == 2
                swap(nums, p1, p2);
                p2 -= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
    }
}
