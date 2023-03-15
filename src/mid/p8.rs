struct Solution;

impl Solution {
    // 有前导空格、非数字字符
    // 有 + - 号
    pub fn my_atoi(s: String) -> i32 {
        let bytes = s.into_bytes();
        let mut signum = 1;
        let mut digit: i32 = 0;
        let mut hit = false;
        for ch in bytes {
            match ch {
                b'0'..=b'9' => {
                    let n = (ch - b'0') as i32 * signum;
                    digit = digit.saturating_mul(10).saturating_add(n);
                    if digit == i32::MAX || digit == i32::MIN {
                        return digit;
                    }
                    if !hit {
                        digit *= signum; // 不可能溢出
                    }
                    hit = true;
                }
                b'-' if !hit => {
                    signum = -1;
                    hit = true;
                }
                b'+' if !hit => {
                    signum = 1;
                    hit = true;
                }
                b' ' if hit => break,
                b' ' => {}
                _ => break,
            }
        }

        digit
    }
}
