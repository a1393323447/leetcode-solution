use std::collections::VecDeque;

struct Solution;

impl Solution {
    // 拓扑排序
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut degree = vec![0; n];
        let mut graph = vec![vec![]; n];
        for r in richer {
            // p > q
            let rich = r[0] as usize;
            let poor = r[1] as usize;
            graph[rich].push(poor);
            degree[poor] += 1;
        }

        let mut q = VecDeque::with_capacity(n);
        let mut ans: Vec<_> = (0..n as i32).collect();
        for (i, &d) in degree.iter().enumerate() {
            if d == 0 {
                q.push_back(i);
            }
        }

        while let Some(rich) = q.pop_front() {
            for &poor in graph[rich].iter() {
                if quiet[ans[rich] as usize] < quiet[ans[poor] as usize] {
                    ans[poor] = ans[rich] as i32;
                }
                degree[poor] -= 1;
                if degree[poor] == 0 {
                    q.push_back(poor);
                }
            }
        }

        ans
    }
}
