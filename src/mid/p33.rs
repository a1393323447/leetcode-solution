struct Solution;

impl Solution {
    // 思路: 先通过二分找到数组分界点, 然后分别在两段数组中二分搜索, 复杂度: O(log n)
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
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
        // 理论上还可以优化, 因为 K 是某段区间的极值, 所以可以和 target 比较, 直接排除某段区间, 只要对于另一段区间进行二分搜索即可
        let k = left; // left == right
        if nums[k] == target {
            return k as i32;
        } else if let Ok(pos) = nums[..k].binary_search(&target) {
            return pos as i32;
        } else if let Ok(pos) = nums[(k + 1)..].binary_search(&target) {
            // 这里 k + 1 不会超限, 因为先检验了 k 是否为答案
            return (pos + k + 1) as i32;
        } else {
            return -1;
        }
    }
}
