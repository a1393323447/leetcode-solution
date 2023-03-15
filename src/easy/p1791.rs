struct Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len();
        let mut cnts = vec![0; n + 2];

        for edge in edges {
            let ui = edge[0] as usize;
            let vi = edge[1] as usize;
            cnts[ui] += 1;
            cnts[vi] += 1;
        }

        cnts.into_iter().position(|cnt| cnt == n).unwrap_or(0) as i32
    }
}
