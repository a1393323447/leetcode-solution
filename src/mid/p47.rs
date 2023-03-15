struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut res = vec![];
        let mut cache = vec![];
        let mut vis = vec![false; nums.len()];

        Solution::permute_unique_impl(&mut cache, &nums, &mut vis, &mut res);

        res
    }

    fn permute_unique_impl(
        cache: &mut Vec<i32>,
        nums: &[i32],
        vis: &mut [bool],
        res: &mut Vec<Vec<i32>>,
    ) {
        if cache.len() == nums.len() {
            res.push(cache.clone());
            return;
        }

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] && !vis[i - 1] {
                continue;
            }

            if !vis[i] {
                vis[i] = true;
                cache.push(nums[i]);

                Solution::permute_unique_impl(cache, nums, vis, res);

                cache.pop();
                vis[i] = false;
            }
        }
    }
}
