struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<_> = nums.into_iter().map(|num| num.to_string()).collect();
        nums.sort_by(|a, b| (b.clone() + a).cmp(&(a.clone() + b)));

        if nums[0].as_bytes().iter().all(|&b| b == b'0') {
            return "0".into();
        } else {
            return nums.join("");
        }
    }
}
