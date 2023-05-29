struct Solution;

impl Solution {
    // 参考 p142
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut s = 0;
        let mut f = 0;

        loop {
            s = nums[s] as usize;
            f = nums[f] as usize;
            f = nums[f] as usize;
            if s == f {
                break;
            }
        }

        // l = a + b
        // when f == s
        // f = s + nb
        // f = 2s
        // f = 2nb
        // s = nb

        f = 0;

        while f != s {
            s = nums[s] as usize;
            f = nums[f] as usize;
        }

        // f == s
        // 定有
        // f = a
        // s = a + nb

        s as i32
    }
}
