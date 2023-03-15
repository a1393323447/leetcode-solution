use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();

        let mut res = vec![];
        let mut cache = vec![];
        Solution::combination_sum_impl(&candidates, target, &mut cache, &mut res);

        res
    }

    pub fn combination_sum_impl(
        candidates: &[i32],
        target: i32,
        cache: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        for (idx, &v) in candidates.iter().enumerate() {
            match v.cmp(&target) {
                std::cmp::Ordering::Less => {
                    // 去重
                    if idx > 0 && v == candidates[idx - 1] {
                        continue;
                    }
                    cache.push(v);
                    Solution::combination_sum_impl(
                        &candidates[(idx + 1)..],
                        target - v,
                        cache,
                        res,
                    );
                    cache.pop();
                }
                std::cmp::Ordering::Equal => {
                    cache.push(v);
                    res.push(cache.clone());
                    cache.pop();
                    return;
                }
                std::cmp::Ordering::Greater => break,
            }
        }
    }
}
