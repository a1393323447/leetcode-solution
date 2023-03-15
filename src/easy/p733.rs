struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let sr = sr as usize;
        let sc = sc as usize;
        let p = image[sr][sc];

        if p == color {
            return image;
        }

        let mut queue = VecDeque::new();
        queue.push_back((sr, sc));
        while let Some((y, x)) = queue.pop_front() {
            if let Some(pc) = image.get_mut(y).map(|row| row.get_mut(x)).flatten() {
                if *pc == p {
                    *pc = color;
                    queue.push_back((y - 1, x));
                    queue.push_back((y + 1, x));
                    queue.push_back((y, x - 1));
                    queue.push_back((y, x + 1));
                }
            }
        }

        image
    }
}
