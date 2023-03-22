struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut s = 0;
        let mut f = 0;
        let mut w_idx = 0;
        loop {
            if f >= chars.len() {
                w_idx = Solution::zip(chars, f, s, w_idx);
                break;
            }
            if chars[s] == chars[f] {
                f += 1;
            } else {
                w_idx = Solution::zip(chars, f, s, w_idx);
                s = f;
            }
        }

        w_idx as i32
    }

    fn zip(chars: &mut Vec<char>, f: usize, s: usize, w: usize) -> usize {
        chars[w] = chars[s];
        
        let len = f - s;

        if len == 1 {
            return w + 1;
        }

        let mut num = len;
        let mut idx = 0;
        let mut digits = [0;20];
        while num != 0 {
            let r = num % 10;
            let q = num / 10;
            digits[idx] = r;

            num = q;
            idx += 1;
        }
        
        let mut w_idx = w + 1;
        let digits = digits[..idx].iter().map(|d| (*d as u8 + b'0') as char).rev();
        for digit in digits {
            chars[w_idx] = digit;
            w_idx += 1;
        }

        w_idx
    }
}