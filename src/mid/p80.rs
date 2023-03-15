struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = nums.len();
        let mut slow = 0;
        let mut fast = 0;
        let mut ele = nums[0];
        let mut cnt = 0;

        while fast < len {
            if nums[fast] == ele {
                if cnt < 2 {
                    cnt += 1;
                    nums[slow] = nums[fast];
                    fast += 1;
                    slow += 1;
                } else {
                    // 元素重复次数超过 2
                    while fast < len && nums[fast] == ele {
                        fast += 1;
                    }
                }
            } else {
                ele = nums[fast];
                cnt = 1;
                nums[slow] = nums[fast];
                fast += 1;
                slow += 1;
            }
        }
        unsafe {
            nums.set_len(slow);
        }
        slow as i32
    }
}
