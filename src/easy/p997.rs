struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut trust_graph = vec![vec![false; n]; n];

        for i in 0..n {
            trust_graph[i][i] = true;
        }

        for t in trust {
            let a = t[0] as usize;
            let b = t[1] as usize;
            // a trust b
            trust_graph[b - 1][a - 1] = true;
        }

        for (idx, person) in trust_graph.iter().enumerate() {
            if person.into_iter().all(|&be_trusted| be_trusted) {
                let mut trust_someone = false;
                for i in 0..n {
                    if i == idx {
                        continue;
                    }
                    trust_someone = trust_someone || trust_graph[i][idx];
                }
                if !trust_someone {
                    return (idx + 1) as i32;
                }
            }
        }

        -1
    }

    pub fn find_judge2(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut trusted = vec![0; n as usize];
        for t in trust {
            trusted[t[0] as usize - 1] -= 1;
            trusted[t[1] as usize - 1] += 1;
        }
        trusted
            .into_iter()
            .position(|t| t == n - 1)
            .map(|i| (i + 1) as i32)
            .unwrap_or(-1)
    }
}
