struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable();
        
        // 可优化为二分查找
        let size = citations.len();
        for (i, v) in citations.into_iter().enumerate() {
            let h = (size - i) as i32;
            if h <= v {
                return h;
            }
        }

        0
    }
}