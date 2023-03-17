struct Solution;

use crate::datastructure::union_find::quick_union::UnionFind;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new(1001);
        let mut res = vec![];
        for edge in edges {
            let p = edge[0] as usize;
            let q = edge[1] as usize;

            let old_count = uf.count();
            uf.union(p, q);
            let cur_count = uf.count();

            if old_count == cur_count {
                res = edge;
            }
        }

        res
    }
}
