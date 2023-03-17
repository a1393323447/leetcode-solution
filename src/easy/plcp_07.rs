struct Solution;

impl Solution {
    pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;

        let mut graph = vec![vec![]; n];
        for edge in relation {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
        }

        let mut cnt = 0;
        Solution::dfs(0, &mut cnt, 0, k, &graph);

        cnt
    }

    fn dfs(node: usize, cnt: &mut i32, cur_step: i32, k: i32, graph: &Vec<Vec<usize>>) {
        if cur_step == k {
            if node == graph.len() - 1 {
                *cnt += 1;
            }
            return;
        }

        for &next in graph[node].iter() {
            Solution::dfs(next, cnt, cur_step + 1, k, graph);
        }
    }
}
