struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut cache = vec![];
        let mut vis = vec![false; nums.len()];

        Solution::permute_impl(&nums, &mut vis, &mut cache, &mut res);

        res
    }

    fn permute_impl(nums: &[i32], vis: &mut [bool], cache: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if cache.len() == nums.len() {
            res.push(cache.clone());
            return;
        }

        for idx in 0..vis.len() {
            if !vis[idx] {
                vis[idx] = true;
                cache.push(nums[idx]);
                Solution::permute_impl(nums, vis, cache, res);
                cache.pop();
                vis[idx] = false;
            }
        }
    }
}
