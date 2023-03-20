struct Solution;

impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let graph = Solution::build_graph(n, paths);

        let mut colors = vec![0; n];
        for i in 0..n {
            if colors[i] == 0 {
                Solution::coloring(i, &mut colors, &graph);
            }
        }

        colors
    }

    fn coloring(node: usize, colors: &mut [i32], graph: &Vec<Vec<usize>>) {
        let mut color_can_use = [0, 1, 2, 3, 4];
        for &next in graph[node].iter() {
            color_can_use[colors[next] as usize] = 0;
        }

        colors[node] = color_can_use.into_iter().find(|c| *c != 0).unwrap();

        for &next in graph[node].iter() {
            if colors[next] == 0 {
                Solution::coloring(next, colors, graph);
            }
        }
    }

    fn build_graph(n: usize, paths: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut graph = vec![Vec::with_capacity(3); n];
        for path in paths {
            let u = path[0] as usize - 1;
            let v = path[1] as usize - 1;
            graph[u].push(v);
            graph[v].push(u);
        }
        graph
    }
}
