struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        if !arr.iter().any(|&v| v == 0) {
            return false;
        }

        let mut vis = vec![false; arr.len()];

        Solution::dfs(&mut vis, &arr, start as usize)
    }

    fn dfs(vis: &mut [bool], arr: &[i32], idx: usize) -> bool {
        if vis[idx] {
            return false;
        }

        if arr[idx] == 0 {
            return true;
        }

        let step = arr[idx] as usize;
        let mut is_found = false;

        vis[idx] = true;
        if step <= idx {
            is_found = Solution::dfs(vis, arr, idx - step);
        }

        if !is_found && step + idx < arr.len() {
            is_found = Solution::dfs(vis, arr, idx + step);
        }

        vis[idx] = false;

        is_found
    }
}
