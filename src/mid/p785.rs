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
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut colors = vec![Color::Uncolored; n];

        let mut i = 0;
        while i < colors.len() {
            if colors[i] == Color::Uncolored && !Solution::dfs(i, Color::Blue, &mut colors, &graph)
            {
                return false;
            }
            i += 1;
        }

        true
    }

    fn dfs(node: usize, pre_color: Color, colors: &mut [Color], graph: &Vec<Vec<i32>>) -> bool {
        let cur_color = pre_color.rev();
        colors[node] = cur_color;

        for &next in graph[node].iter() {
            let next = next as usize;
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
