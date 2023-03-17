use std::collections::VecDeque;

struct Solution;

// 本质上是判断有向图中是否存在环
// 使用拓扑排序
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut degree = vec![0; n];

        // 创建有向图,并初始化入度数组
        let mut graph = vec![vec![]; n];
        for it in prerequisites.iter() {
            let (from, to) = (it[1] as usize, it[0] as usize);
            graph[from].push(to);
            degree[to] += 1;
        }

        // 拓扑排序
        let mut queue = VecDeque::new();
        for (i, &d) in degree.iter().enumerate() {
            if d == 0 {
                queue.push_back(i);
            }
        }

        while let Some(cur) = queue.pop_front() {
            for &next in graph[cur].iter() {
                degree[next] -= 1;
                if degree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        degree.into_iter().all(|d| d == 0)
    }
}
