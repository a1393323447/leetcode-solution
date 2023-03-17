use crate::datastructure::union_find::quick_union::UnionFind;

struct Solution;

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut uf = UnionFind::new(27);
        let mut checks = vec![];
        for eq in equations {
            let eq = eq.as_bytes();
            let u = (eq[0] - b'0') as usize;
            let v = (eq[3] - b'0') as usize;
            let c = eq[1];
            if c == b'!' {
                checks.push((u, v));
            } else {
                uf.union(u, v);
            }
        }

        for (u, v) in checks {
            if uf.find(u, v) {
                return false;
            }
        }

        true
    }
}
