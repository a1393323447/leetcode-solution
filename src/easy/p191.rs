struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        n.count_ones() as i32
    }

    pub fn hammingWeight2(n: u32) -> i32 {
        let m1 = 0x55555555;
        let m2 = 0x33333333;
        let m3 = 0x0f0f0f0f;
        let m4 = 0x01010101;

        let mut cnt = n - ((n >> 1) & m1);
        cnt = (cnt & m2) + ((cnt >> 2) & m2);
        cnt = ((cnt >> 4) + cnt) & m3;
        cnt *= m4;

        (cnt >> 24) as i32
    }
}
