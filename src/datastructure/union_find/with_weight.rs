pub struct UnionFind {
    tree: Vec<usize>,
    weights: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        UnionFind {
            tree: (0..size).collect(),
            weights: vec![1; size],
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.get_root(p);
        let root_q = self.get_root(q);

        if root_p == root_q {
            return;
        }

        if self.weights[root_p] < self.weights[root_q] {
            self.tree[root_p] = root_q;
            self.weights[root_q] += self.weights[root_p];
        } else {
            self.tree[root_q] = root_p;
            self.weights[root_p] += self.weights[root_q];
        }
    }

    pub fn max_count(&self) -> usize {
        self.weights.iter().max().map(|w| *w).unwrap()
    }

    pub fn reset(&mut self) {
        for (i, r) in self.tree.iter_mut().enumerate() {
            *r = i;
        }
        self.weights.fill(1);
    }

    fn get_root(&self, i: usize) -> usize {
        let mut i = i;
        while self.tree[i] != i {
            i = self.tree[i];
        }

        i
    }

    fn find(&self, p: usize, q: usize) -> bool {
        self.get_root(p) == self.get_root(q)
    }

    fn max_weights(&self) -> usize {
        self.weights.iter().max().map(|w| *w).unwrap()
    }
}
