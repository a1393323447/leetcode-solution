struct Solution;

use crate::datastructure::union_find::quick_union::UnionFind;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(is_connected.len());
        for (q, connection) in is_connected.into_iter().enumerate() {
            for (p, flag) in connection.into_iter().enumerate() {
                if flag == 1 {
                    uf.union(p, q);
                }
            }
        }

        uf.count() as i32
    }
}
