struct Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let idx = match intervals.binary_search_by(|a| a[0].cmp(&new_interval[0])) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        intervals.insert(idx, new_interval);

        let mut res = vec![intervals[0].clone()];
        for insert_interval in intervals {
            let last_interval = res.last_mut().unwrap();
            if Solution::is_intersect(last_interval, &insert_interval) {
                let new_interval = Solution::merge_intervals(last_interval, &insert_interval);
                last_interval[..2].copy_from_slice(&new_interval);
            } else {
                res.push(insert_interval);
            }
        }

        res
    }

    fn is_intersect(a: &[i32], b: &[i32]) -> bool {
        a[1] >= b[0]
    }

    fn merge_intervals(a: &[i32], b: &[i32]) -> [i32; 2] {
        let min = a[0].min(b[0]);
        let max = a[1].max(b[1]);

        [min, max]
    }
}
