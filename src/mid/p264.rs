use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

struct Solution;

// 最小堆解法
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let factors = [2, 3, 5];
        let mut seq: BinaryHeap<_> = vec![Reverse(1usize)].into_iter().collect();
        let mut vis: HashSet<_> = vec![1usize].into_iter().collect();
        for _ in 1..n {
            let min = seq.pop().unwrap().0;
            for f in factors.clone() {
                let value = f * min;
                if vis.insert(value) {
                    seq.push(Reverse(value));
                }
            }
        }

        seq.pop().unwrap().0 as i32
    }
}

// dp 解法, 和最小堆解法本质上是相同的, 都是在尝试找出乘积的最小值作为第 i 个丑数
// 这个解法的原理是：
// - 用一个数组dp存储丑数，dp[i]表示第i+1个丑数（i从0开始）。
// - 用三个指针p2,p3,p5分别指向dp中的某个元素，表示下一个要乘以2,3,5的丑数。
// - 每次从dp[p2]*2, dp[p3]*3, dp[p5]*5中选出最小的一个作为新的丑数，并将对应的指针后移一位。
//   重复这个过程直到找到第n个丑数。
// 这个解法的正确性是基于以下事实：
// - 任何一个丑数都可以由它前面的某个丑数乘以2,3,5得到。
// - 通过三个指针，我们可以保证每次生成的新丑数都是按照从小到大的顺序排列，并且不会遗漏或重复任何一个丑数。
impl Solution {
    pub fn nth_ugly_number2(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[1] = 1;

        let mut i = 2;
        let mut p1 = 1;
        let mut p2 = 1;
        let mut p3 = 1;
        while i <= n {
            let n1 = dp[p1] * 2;
            let n2 = dp[p2] * 3;
            let n3 = dp[p3] * 5;
            dp[i] = n1.min(n2).min(n3);
            if n1 == dp[i] {
                p1 += 1;
            } else if n2 == dp[i] {
                p2 += 1;
            } else {
                p3 += 1;
            }
            if dp[i] == dp[i - 1] {
                i -= 1;
            }
            i += 1;
        }

        dp[n]
    }
}
