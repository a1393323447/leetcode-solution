struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;

        while left < right {
            let width = (right - left) as i32;
            if height[left] < height[right] {
                let area = height[left] * width;
                max_area = max_area.max(area);
                left += 1;
            } else {
                let area = height[right] * width;
                max_area = max_area.max(area);
                right -= 1;
            }
        }
        max_area
    }
}
