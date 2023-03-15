struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let signum = x.signum();
        let mut nums = [0i32; 10];
        let mut idx = 0;
        while x != 0 {
            nums[idx] = x % 10 * signum;
            x /= 10;
            idx += 1;
        }
        nums[..idx]
            .iter()
            .try_fold(0i32, |acc, &n| acc.checked_mul(10)?.checked_add(n))
            .map(|n| n * signum)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(i32::MAX), 0);
        assert_eq!(Solution::reverse(i32::MIN), 0);
    }
}
