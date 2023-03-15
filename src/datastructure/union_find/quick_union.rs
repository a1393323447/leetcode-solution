pub struct UnionFind {
    tree: Vec<usize>,
    cnt: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        UnionFind {
            tree: (0..size).collect(),
            cnt: size,
        }
    }

    pub fn count(&self) -> usize {
        self.cnt
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.get_root(p);
        let root_q = self.get_root(q);

        if root_p == root_q {
            return ;
        }

        self.cnt -= 1;
        self.tree[root_p] = root_q;
    }

    pub fn reset(&mut self) {
        for (i, r) in self.tree.iter_mut().enumerate() {
            *r = i;
        }
    }

    pub fn get_root(&self, i: usize) -> usize {
        let mut i = i;
        while self.tree[i] != i {
            i = self.tree[i];
        }

        i
    }

    pub fn find(&mut self, p: usize, q: usize) -> bool {
        self.get_root(p) == self.get_root(q)
    }
}
