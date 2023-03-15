struct Solution;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for v in heights.iter() {
            for &num in v.iter() {
                min = min.min(num);
                max = max.max(num);
            }
        }

        let mut left = min;
        let mut right = max;

        while left < right {
            let mid = (left + right) / 2;
            if Solution::has_path(&heights, mid) {
                if left == mid {
                    break;
                }
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        left
    }

    fn has_path(heights: &Vec<Vec<i32>>, delta: i32) -> bool {
        Solution::dfs(heights, heights[0][0], (0, 0), delta)
    }

    fn dfs(heights: &Vec<Vec<i32>>, prev_height: i32, (x, y): (usize, usize), delta: i32) -> bool {
        const IS_VIS: i32 = 0;

        let n = heights.len();
        let m = heights[0].len();
        if (x, y) == (m - 1, n - 1) {
            return true;
        }

        let val = heights.get(y).map(|v| v.get(x)).flatten();

        if let Some(height) = val {
            if *height == IS_VIS {
                return false;
            }

            if (prev_height - *height).abs() > delta {
                return false;
            }

            let h = *height;

            let ptr = height as *const i32 as *mut i32;
            unsafe {
                *ptr = IS_VIS;
            }

            let res = Solution::dfs(heights, h, (x, y - 1), delta)
                || Solution::dfs(heights, h, (x, y + 1), delta)
                || Solution::dfs(heights, h, (x - 1, y), delta)
                || Solution::dfs(heights, h, (x + 1, y), delta);

            unsafe {
                *ptr = h;
            }

            return res;
        }

        false
    }
}
