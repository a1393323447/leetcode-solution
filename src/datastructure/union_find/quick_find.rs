pub struct UnionFind {
    tree: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        UnionFind {
            tree: (0..size).collect(),
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.get_root(p);
        let root_q = self.get_root(q);

        if root_p == root_q {
            return;
        }

        self.tree[root_p] = root_q;

        for parent in &mut self.tree {
            if *parent == root_p {
                *parent = root_q;
            }
        }
    }

    pub fn reset(&mut self) {
        for (i, r) in self.tree.iter_mut().enumerate() {
            *r = i;
        }
    }

    pub fn get_root(&mut self, i: usize) -> usize {
        self.tree[i]
    }

    pub fn find(&mut self, p: usize, q: usize) -> bool {
        self.get_root(p) == self.get_root(q)
    }
}
