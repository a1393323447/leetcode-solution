struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        // 我的评价是远不如标准库
        let x_half = 0.5 * x as f64;
        let mut i = (x as f64).to_bits();
        i = 0x1FF7A3BEA91D9B1B + (i >> 1);
        let mut f = f64::from_bits(i);
        f = 0.5 * f + x_half / f;
        f = 0.5 * f + x_half / f;
        f = 0.5 * f + x_half / f;
        f as i32
    }
}
