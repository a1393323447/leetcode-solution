struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for idx in 0..numbers.len() {
            let num = numbers[idx];
            if let Ok(t_idx) = numbers[(idx + 1)..].binary_search(&(target - num)) {
                return vec![(idx + 1) as i32, (t_idx + idx + 2) as i32];
            }
        }
        unreachable!()
    }
}

// 类似于二分的思路
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut a, mut b) = (0, numbers.len() - 1);
    loop {
        match target.cmp(&(numbers[a] + numbers[b])) {
            std::cmp::Ordering::Equal => return vec![a as i32 + 1, b as i32 + 1],
            std::cmp::Ordering::Greater => a += 1,
            std::cmp::Ordering::Less => b -= 1,
        }
    }
}
