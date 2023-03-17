use crate::traites::ToReverse;
use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

#[derive(Clone, Copy)]
struct Info {
    to: usize,
    weight: i32,
}

impl PartialEq for Info {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for Info {}

impl PartialOrd for Info {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Ord for Info {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        const IF: i32 = i32::MAX;
        let mut graph = vec![vec![IF; n + 1]; n + 1];
        for edge in times {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let weight = edge[2];
            graph[u][v] = weight;
        }

        let mut ordered = graph[k].clone();
        ordered[k] = 0;
        let mut unordered = BinaryHeap::with_capacity(n - 1);
        for (to, &weight) in graph[k].iter().enumerate() {
            let info = Info { to, weight };
            unordered.push(info.rev());
        }

        while let Some(Reverse(Info {
            to: q,
            weight: new_k_to_q_weight,
        })) = unordered.pop()
        {
            let old_k_to_q_weight = ordered[q];
            if old_k_to_q_weight < new_k_to_q_weight {
                continue;
            }
            let k_to_q_weight = new_k_to_q_weight;
            for (p, &q_to_p_weight) in graph[q].iter().enumerate() {
                if q_to_p_weight == IF {
                    continue;
                }
                let new_k_to_p_weight = k_to_q_weight + q_to_p_weight;
                let old_k_to_p_weight = ordered[p];
                if new_k_to_p_weight < old_k_to_p_weight {
                    ordered[p] = new_k_to_p_weight;
                    unordered.push(
                        Info {
                            to: p,
                            weight: new_k_to_p_weight,
                        }
                        .rev(),
                    );
                }
            }
        }

        ordered[1..=n]
            .into_iter()
            .max()
            .map(|&w| if w == IF { -1 } else { w })
            .unwrap()
    }
}
