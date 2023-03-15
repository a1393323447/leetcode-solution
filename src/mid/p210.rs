use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;

        let mut graph = vec![vec![]; n];
        let mut indegree = vec![0; n];
        for edge in prerequisites.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[v].push(u);
            indegree[v] += 1;
        }

        let mut res = vec![];
        let mut q = VecDeque::with_capacity(n);
        for (i, &d) in indegree.iter().enumerate() {
            if d == 0 {
                res.push(i as i32);
                q.push_back(i);
            }
        }

        while let Some(node) = q.pop_front() {
            for &to in graph[node].iter() {
                indegree[to] -= 1;
                if indegree[to] == 0 {
                    res.push(to as i32);
                    q.push_back(to);
                }
            }
        }

        if res.len() == n {
            res
        } else {
            res.clear();
            res
        }
    }
}