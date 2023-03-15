struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        if len == 1 {
            return;
        }

        let mut idx = len - 1;
        while idx > 0 {
            if nums[idx] > nums[idx - 1] {
                nums[idx..].sort();
                let pos = match nums[idx..].binary_search(&nums[idx - 1]) {
                    Ok(i) => {
                        let mut pos = i + idx;
                        let val = nums[pos];
                        while nums[pos] == val {
                            pos += 1;
                        }
                        pos
                    }
                    Err(i) => i + idx,
                };
                let t = nums[idx - 1];
                nums[idx - 1] = nums[pos];
                nums[pos] = t;
                return;
            }
            idx -= 1;
        }

        nums.reverse()
    }
}
