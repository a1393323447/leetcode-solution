use std::collections::VecDeque;

struct Solution;

const LAND: i32 = 1;
const WATER: i32 = 0;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let row_size = grid.len();
        let col_size = grid[0].len();

        let mut max_area = 0;
        for y in 0..row_size {
            for x in 0..col_size {
                if grid[y][x] == LAND {
                    let area = Solution::area_of_island(&mut grid, (y, x));
                    max_area = max_area.max(area);
                }
            }
        }

        max_area
    }

    fn area_of_island(grid: &mut Vec<Vec<i32>>, pos: (usize, usize)) -> i32 {
        let mut area = 0;
        let mut queue = VecDeque::new();
        queue.push_back(pos);
        while let Some((y, x)) = queue.pop_front() {
            if let Some(ele) = grid.get_mut(y).map(|row| row.get_mut(x)).flatten() {
                if *ele == LAND {
                    area += 1;
                    *ele = WATER;
                    [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
                        .into_iter()
                        .for_each(|pos| queue.push_back(pos));
                }
            }
        }

        area
    }
}
