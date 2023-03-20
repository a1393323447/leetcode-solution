struct Solution;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Blue,
    Uncolored,
}

impl Color {
    fn rev(self) -> Color {
        match self {
            Color::Red => Color::Blue,
            Color::Blue => Color::Red,
            Color::Uncolored => Color::Uncolored,
        }
    }
}

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;

        let graph = Solution::build_graph(n + 1, dislikes);

        let mut colors = vec![Color::Uncolored; n + 1];
        let mut i = 1;
        while i < colors.len() {
            if colors[i] == Color::Uncolored && !Solution::dfs(i, Color::Blue, &mut colors, &graph)
            {
                return false;
            }
            i += 1;
        }

        true
    }

    fn build_graph(n: usize, dislikes: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut graph = vec![vec![]; n];

        for dislike in dislikes {
            let a = dislike[0] as usize;
            let b = dislike[1] as usize;
            graph[a].push(b);
            graph[b].push(a);
        }

        graph
    }

    fn dfs(node: usize, pre_color: Color, colors: &mut [Color], graph: &Vec<Vec<usize>>) -> bool {
        let cur_color = pre_color.rev();
        colors[node] = cur_color;

        for &next in graph[node].iter() {
            let next_color = colors[next];

            if next_color == Color::Uncolored {
                let is_bipartite = Solution::dfs(next, cur_color, colors, graph);
                if !is_bipartite {
                    return false;
                }
            } else if cur_color == next_color {
                return false;
            }
        }

        true
    }
}
