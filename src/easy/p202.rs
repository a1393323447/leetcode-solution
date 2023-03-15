struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut set = HashSet::new();
        loop {
            let mut sum = 0;
            loop {
                let digit = n % 10;
                sum += digit.pow(2);

                n /= 10;
                if n == 0 {
                    break;
                }
            }

            if sum == 1 {
                return true;
            } else if set.get(&sum).is_none() {
                set.insert(sum);
            } else {
                return false;
            }

            n = sum;
        }
    }
}

// 快慢指针
// 从题目上看, 快乐数所生成的数字序列最终会到达 1 并死循环
// 非快乐数不会到达一, 然后循环
// 所以数字序列会形成一个环状, 所以可以用快慢指针
pub fn is_happy(n: i32) -> bool {
    #[inline]
    fn next(v: &mut i32) {
        let (mut m, mut n) = (*v, 0i32);
        while m > 0 {
            let t = m % 10;
            n += t * t;
            m /= 10;
        }
        *v = n;
    }
    let (mut s, mut f) = (n, n);
    loop {
        next(&mut s);
        next(&mut f);
        next(&mut f);
        if s == f {
            return f == 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(2), false);
    }
}
