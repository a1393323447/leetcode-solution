struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let r = matrix.len();
        let c = matrix[0].len();

        let mut r0 = false;
        let mut c0 = false;

        for ci in 0..c {
            if matrix[0][ci] == 0 {
                r0 = true;
                break;
            }
        }

        for ri in 0..r {
            if matrix[ri][0] == 0 {
                c0 = true;
                break;
            }
        }

        for ri in 1..r {
            for ci in 1..c {
                if matrix[ri][ci] == 0 {
                    matrix[ri][0] = 0;
                    matrix[0][ci] = 0;
                }
            }
        }

        for ri in 1..r {
            if matrix[ri][0] == 0 {
                matrix[ri].fill(0);
            }
        }

        for ci in 1..c {
            if matrix[0][ci] == 0 {
                for ri in 1..r {
                    matrix[ri][ci] = 0;
                }
            }
        }

        if r0 {
            matrix[0].fill(0);
        }

        if c0 {
            for ri in 0..r {
                matrix[ri][0] = 0;
            }
        }
    }
}
