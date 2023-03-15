struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        let mut cache = String::new();
        Solution::generate(n, n, &mut cache, &mut res);

        res
    }

    fn generate(left: i32, right: i32, cache: &mut String, res: &mut Vec<String>) {
        if left == 0 && right == 0 {
            res.push(cache.clone());
            return;
        }
        if left <= right && left > 0 {
            cache.push('(');
            Solution::generate(left - 1, right, cache, res);
            cache.pop();
        }
        if left <= right - 1 && right > 0 {
            cache.push(')');
            Solution::generate(left, right - 1, cache, res);
            cache.pop();
        }
    }
}
