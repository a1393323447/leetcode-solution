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
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        const VIS: i32 = i32::MAX;
        let size = matrix.len() * matrix[0].len();

        let mut dir = (0, 1);
        let mut pos = (0, 0);
        let mut count = 0;
        let mut res = vec![];
        while count < size {
            match matrix
                .get_mut(pos.0)
                .map(|row| row.get_mut(pos.1))
                .flatten()
            {
                Some(num) if *num != VIS => {
                    res.push(*num);
                    *num = VIS;
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

        res
    }
}
