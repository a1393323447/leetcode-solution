struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut cache = vec![0];
        let mut res = vec![];

        Solution::dfs(&graph, 0, &mut cache, &mut res);

        res
    }

    fn dfs(graph: &Vec<Vec<i32>>, node: usize, cache: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if node == graph.len() - 1 {
            res.push(cache.clone());
            return;
        }

        for &next in graph[node].iter() {
            cache.push(next);
            Solution::dfs(graph, next as usize, cache, res);
            cache.pop();
        }
    }
}
