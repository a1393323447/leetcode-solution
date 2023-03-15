struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return 0;
        }

        let mut step = 0;
        let mut idx = 0;
        loop {
            let max_idx = idx + nums[idx] as usize;
            if max_idx >= len - 1 {
                return step + 1;
            }

            idx = nums[(idx + 1)..=max_idx]
                .iter()
                .enumerate()
                .map(|(i, &s)| (i + idx + 1, s as usize + i + idx))
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap()
                .0;

            step += 1;
        }
    }
}
