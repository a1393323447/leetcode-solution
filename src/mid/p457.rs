struct Solution;

impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut vis = vec![false; len];
        for i in 0..len {
            if !vis[i] && Solution::walk(i, &nums, &mut vis) {
               return true;
            }
        }
        
        false
    }

    fn walk(start: usize, nums: &[i32], vis: &mut [bool]) -> bool {
        let len = nums.len() as i32;

        let mut f = start;
        let mut s = start;

        let sign = nums[s as usize].signum();
        
        let mut flag = true;
        let next = |cur: usize| {
            let step = nums[cur] % len;
            ((cur as i32 + step + len) % len) as usize
        };
        let mut update_flag = 
        |f: usize| if flag {
            flag = sign == nums[f].signum();
        };

        loop {
            s = next(s);

            update_flag(f);
            vis[f] = true;
            f = next(f);
            
            update_flag(f);
            vis[f] = true;
            f = next(f);

            if f == s {
                vis[f] = true;
                // 检查 seq 是否满足条件
                let mut flag = true;
                let sign = nums[f].signum();
                let mut loop_len = 1;
                f = next(f);
                while f != s {
                    if flag {
                        flag = sign == nums[f].signum();
                    }
                    loop_len += 1;
                    f = next(f);
                }
                if flag {
                    flag = sign == nums[f].signum();
                }
                return flag && loop_len > 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let nums = vec![-1,-2,-3,-4,-5];
        Solution::circular_array_loop(nums);
    }
}