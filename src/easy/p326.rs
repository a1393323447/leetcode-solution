struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        let mut n = n;
        while n % 3 == 0 {
            n /= 3;
        }

        n == 1
    }
    // 我们还可以使用一种较为取巧的做法。
    // 在题目给定的 32 位有符号整数的范围内，最大的 3 的幂为 3^19=1162261467。
    // 我们只需要判断 n 是否是 3^19 的约数即可。
    pub fn is_power_of_three2(n: i32) -> bool {
        n > 0 && 1162261467 % n == 0
    }
    // 还可以打表: 列出范围内所有的 3 的幂
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::is_power_of_three(27), true);
    }
}
