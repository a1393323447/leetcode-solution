use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut uf = UnionFind::new();
        for &num in nums.iter() {
            uf.union(num, num + 1);
        }

        let mut max_len = 1;
        for num in nums {
            let r = uf.get_root(num);
            let cur_len = r - num;
            max_len = max_len.max(cur_len);
        }

        max_len
    }

    // 逃课版
    pub fn longest_consecutive2(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        nums.sort();

        let mut max_len = 1;
        let mut cur_len = 1;

        for i in 1..nums.len() {
            if nums[i - 1] + 1 == nums[i] {
                cur_len += 1;
            } else if nums[i - 1] == nums[i] {
                continue;
            } else {
                max_len = max_len.max(cur_len);
                cur_len = 1;
            }
        }

        max_len = max_len.max(cur_len);

        max_len as i32
    }
}

use std::hash::Hash;

struct UnionFind<T> {
    tree: HashMap<T, T>,
}

impl<T> UnionFind<T>
where
    T: Hash + Eq + Clone + Copy,
{
    fn new() -> UnionFind<T> {
        UnionFind {
            tree: HashMap::new(),
        }
    }

    fn union(&mut self, p: T, q: T) {
        let root_p = self.get_root(p);
        let root_q = self.get_root(q);

        *self.tree.entry(root_p).or_insert(root_q) = root_q;
    }

    fn get_root(&mut self, q: T) -> T {
        let mut root = q;
        while let Some(&r) = self.tree.get(&root) {
            if r == root {
                break;
            }
            root = r;
        }

        let res = root;

        root = q;
        while let Some(r) = self.tree.get_mut(&root) {
            if *r == root {
                break;
            }
            root = *r;
            *r = res;
        }

        res
    }
}
