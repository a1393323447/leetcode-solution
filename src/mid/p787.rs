struct Solution;

#[derive(Clone, Copy)]
struct Info {
    node: usize,
    step: i32,
    price: i32,
}

// 暴力 dfs: 超时
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let graph = Solution::build_graph(n, flights);
        let mut vis = vec![false; n];
        let cur_info = Info {
            node: src as usize,
            step: 0,
            price: 0,
        };
        let mut res_info = Info {
            node: dst as usize,
            step: k,
            price: i32::MAX,
        };
        Solution::dfs(cur_info, &mut res_info, &graph, &mut vis);

        if res_info.price == i32::MAX {
            -1
        } else {
            res_info.price
        }
    }

    fn build_graph(n: usize, flights: Vec<Vec<i32>>) -> Vec<Vec<(usize, i32)>> {
        let mut graph = vec![vec![]; n];
        for flight in flights {
            let s = flight[0] as usize;
            let d = flight[1] as usize;
            let p = flight[2];
            graph[s].push((d, p));
        }

        graph
    }

    fn dfs(cur: Info, res: &mut Info, graph: &Vec<Vec<(usize, i32)>>, vis: &mut [bool]) {
        if cur.step > res.step {
            return;
        }

        if cur.node == res.node {
            res.price = res.price.min(cur.price);
            return;
        }

        for &(next, price) in graph[cur.node].iter() {
            if !vis[next] {
                vis[next] = true;
                let next_info = Info {
                    node: next,
                    step: cur.step + 1,
                    price: cur.price + price,
                };
                Solution::dfs(next_info, res, graph, vis);
                vis[next] = false;
            }
        }
    }
}

// dp:
// Bellman-Ford 算法: 通过 m 次迭代求出从源点到终点不超过m条边构成的最短路的路径
impl Solution {
    pub fn find_cheapest_price2(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        // i32::MAX + 正数会溢出为负数, 影响判断
        const INF: i32 = 0x3f3f3f3f;

        let n = n as usize;
        let src = src as usize;
        let dst = dst as usize;
        let mut prices = vec![INF; n];
        prices[src] = 0;

        for _ in 0..=k {
            // 为了满足最多只有 k 条变的限制,
            // 所以每一轮循环都要严格基于上一轮循环的值进行更新
            // 如果不是, 则很可能会出现一次迭代中:
            // price[1] = old_price[0] + 1
            // price[2] = price[1] + 1
            // 这就导致了 price[2] 在这一轮循环中实际上是用了两条边
            let old_prices = prices.clone();
            for flight in flights.iter() {
                let s = flight[0] as usize;
                let d = flight[1] as usize;
                let p = flight[2];
                prices[d] = prices[d].min(old_prices[s] + p);
            }
        }

        if prices[dst] == INF {
            -1
        } else {
            prices[dst]
        }
    }
}
