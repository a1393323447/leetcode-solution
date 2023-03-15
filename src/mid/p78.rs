struct Solution;

impl Solution {
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut res = vec![];
        let mut cache = vec![];

        Solution::subsets_impl(&nums, &mut cache, &mut res);

        res
    }

    fn subsets_impl(nums: &[i32], cache: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if nums.is_empty() {
            res.push(cache.clone());
            return;
        }

        Solution::subsets_impl(&nums[1..], cache, res);

        cache.push(nums[0]);
        Solution::subsets_impl(&nums[1..], cache, res);
        cache.pop();
    }
}
