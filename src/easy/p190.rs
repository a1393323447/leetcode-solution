struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        x.reverse_bits()
    }

    pub fn reverse_bits2(x: u32) -> u32 {
        let m1 = 0x55555555;
        let m2 = 0x33333333;
        let m4 = 0x0f0f0f0f;
        let m8 = 0x00ff00ff;

        let mut x = x >> 1 & m1 | (x & m1) << 1;
        x = x >> 2 & m2 | (x & m2) << 2;
        x = x >> 4 & m4 | (x & m4) << 4;
        x = x >> 8 & m8 | (x & m8) << 8;

        x >> 16 & 0x0000ffff | (x & 0x0000ffff) << 16
    }
}
