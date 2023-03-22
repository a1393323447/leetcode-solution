struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        
        let mut n = n;

        while n % 5 == 0 {
            n /= 5;
        }

        while n % 3 == 0 {
            n /= 3;
        }

        while n % 2 == 0 {
            n /= 2;
        }

        n == 1
    }
}