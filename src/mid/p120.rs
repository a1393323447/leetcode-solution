struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let rn = triangle.len();
        let mut dp = triangle.clone();

        for ri in 1..rn {
            let cn = ri + 1;
            for ci in 0..cn {
                let a = dp[ri - 1].get(ci - 1);
                let b = dp[ri - 1].get(ci);
                let min = match (a, b) {
                    (None, None) => unreachable!(),
                    (None, Some(&b)) => b,
                    (Some(&a), None) => a,
                    (Some(&a), Some(&b)) => a.min(b),
                };
                dp[ri][ci] = min + triangle[ri][ci];
            }
        }

        *dp[rn - 1].iter().min().unwrap()
    }
}
