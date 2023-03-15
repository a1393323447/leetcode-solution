struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut cache = vec![];

        Solution::combine_impl(1, &mut cache, k as usize, n, &mut res);

        res
    }

    fn combine_impl(c: i32, cache: &mut Vec<i32>, k: usize, n: i32, res: &mut Vec<Vec<i32>>) {
        if cache.len() == k {
            res.push(cache.clone());
            return;
        }

        for i in c..=n {
            cache.push(i);
            Solution::combine_impl(i + 1, cache, k, n, res);
            cache.pop();
        }
    }
}
