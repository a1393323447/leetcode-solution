struct Solution;

fn turn_right(dir: (isize, isize)) -> (isize, isize) {
    match dir {
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        _ => unreachable!(),
    }
}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0; n]; n];
        Solution::spiral_fill(&mut matrix);

        matrix
    }

    pub fn spiral_fill(matrix: &mut Vec<Vec<i32>>) {
        const NOT_VIS: i32 = 0;
        let size = matrix.len() * matrix[0].len();

        let mut dir = (0, 1);
        let mut pos = (0, 0);
        let mut count = 0;
        while count < size {
            match matrix
                .get_mut(pos.0)
                .map(|row| row.get_mut(pos.1))
                .flatten()
            {
                Some(num) if *num == NOT_VIS => {
                    *num = count as i32 + 1;
                    pos.0 = (pos.0 as isize + dir.0) as usize;
                    pos.1 = (pos.1 as isize + dir.1) as usize;
                    count += 1;
                }
                _ => {
                    pos.0 = (pos.0 as isize - dir.0) as usize;
                    pos.1 = (pos.1 as isize - dir.1) as usize;
                    dir = turn_right(dir);
                    pos.0 = (pos.0 as isize + dir.0) as usize;
                    pos.1 = (pos.1 as isize + dir.1) as usize;
                }
            }
        }
    }
}
