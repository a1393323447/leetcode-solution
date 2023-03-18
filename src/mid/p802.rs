use std::collections::VecDeque;

struct Solution;

impl Solution {
    // 本质上是求不在环中的元素
    // 反图 + 拓扑排序
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();

        let mut q = VecDeque::with_capacity(n / 2);
        // 构造反图 并 初始化反图入度数组 (相当于正图的出度)
        let mut degree = vec![0; n];
        let mut rgraph = vec![vec![]; n];
        for (p, g) in graph.into_iter().enumerate() {
            let out_d = g.len();
            degree[p] = out_d;
            // 初始化队列, 将反图中入度为 0 的节点入队
            if out_d == 0 {
                q.push_back(p);
            }
            for q in g {
                let q = q as usize;
                // 正图: p -> q
                // 反图: q -> p
                rgraph[q].push(p);
            }
        }

        while let Some(node) = q.pop_front() {
            for &to in rgraph[node].iter() {
                degree[to] -= 1;
                if degree[to] == 0 {
                    q.push_back(to);
                }
            }
        }

        // 能进入反图的拓扑排序, 则该节点在原图中一定是环上元素
        degree
            .into_iter()
            .enumerate()
            .filter_map(|(i, d)| if d == 0 { Some(i as i32) } else { None })
            .collect()
    }
}
