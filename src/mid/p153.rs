struct Solution;

impl Solution {
    // 思路: 先通过二分找到数组分界点, 然后分别在两段数组中二分搜索, 复杂度: O(log n)
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            // 直到搜索区间缩成一个点, 该点就为数组分界点
            let mid = (left + right) / 2;
            if nums[left] < nums[mid] {
                // [left, mid] 符合升序, 证明数组分解点在另一半区间 [mid, right] 中
                left = mid;
            } else {
                // [mid, right] 符合升序, 证明数组分解点在另一半区间 [left, mid] 中
                right = mid;
            }
        }

        let k = left; // left == right

        if k < nums.len() - 1 {
            nums[0].min(nums[k + 1])
        } else {
            nums[0]
        }
    }
}
