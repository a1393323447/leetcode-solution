struct Solution;

const WATER: char = '0';
const LAND: char = '1';
const FOUND: char = '2';

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut res = 0;
        for y in 0..m {
            for x in 0..n {
                if grid[y][x] == LAND {
                    res += 1;
                    Solution::fill_island(&mut grid, (x, y));
                }
            }
        }

        res
    }

    pub fn fill_island(grid: &mut Vec<Vec<char>>, start_pos: (usize, usize)) {
        let mut stack = vec![start_pos];
        while let Some((x, y)) = stack.pop() {
            if let Some(ch) = grid.get_mut(y).map(|v| v.get_mut(x)).flatten() {
                if *ch == LAND {
                    *ch = FOUND;
                    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                        .iter()
                        .for_each(|&pos| stack.push(pos));
                }
            }
        }
    }
}
