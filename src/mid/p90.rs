struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut res = vec![];
        let mut cache = vec![];

        Solution::subsets_impl(&nums, &mut cache, &mut res);

        res
    }

    fn subsets_impl(nums: &[i32], cache: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        res.push(cache.clone());

        if nums.is_empty() {
            return;
        }

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            cache.push(nums[i]);
            Solution::subsets_impl(&nums[(i + 1)..], cache, res);
            cache.pop();
        }
    }
}
