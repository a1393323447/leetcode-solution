struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 1 {
            return vec![vec![1]];
        } else if num_rows == 2 {
            return vec![vec![1], vec![1, 1]];
        }

        let mut res = vec![vec![1], vec![1, 1]];
        let num_rows = num_rows as usize;
        for row in 2..num_rows {
            let pre = &res[row - 1];
            let mut cur = vec![];
            cur.push(1);
            pre.windows(2).for_each(|pair| cur.push(pair.iter().sum()));
            cur.push(1);

            res.push(cur);
        }

        res
    }
}
