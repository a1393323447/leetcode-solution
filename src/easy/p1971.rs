use crate::datastructure::union_find::quick_union::UnionFind;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    // bfs
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let source = source as usize;
        let destination = destination as usize;
        let mut vis = vec![false; n];
        let mut q = VecDeque::with_capacity(n);
        q.push_back(source);

        while let Some(node) = q.pop_front() {
            if node == destination {
                return true;
            }
            for &next in graph[node].iter() {
                if !vis[next] {
                    vis[next] = true;
                    q.push_back(next);
                }
            }
        }

        false
    }

    pub fn valid_path2(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut uf = UnionFind::new(n as usize);

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            uf.union(u, v);
        }

        let s = source as usize;
        let d = destination as usize;

        uf.find(s, d)
    }
}
