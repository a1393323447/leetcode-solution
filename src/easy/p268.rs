struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let sum = n * (n + 1) / 2;

        nums.into_iter().fold(sum as i32, |acc, num| acc - num)
    }

    // 位运算
    pub fn missing_number2(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        nums.into_iter()
            .enumerate()
            .fold(n, |acc, (i, num)| acc ^ (i as i32) ^ num)
    }
}
