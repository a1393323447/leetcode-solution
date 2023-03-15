struct Solution;

impl Solution {
    fn map(ch: char) -> i32 {
        match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Unexpected letter {ch}"),
        }
    }
    pub fn roman_to_int(s: String) -> i32 {
        let mut max = 0;
        s.chars().rev().fold(0, |acc, ch| {
            let num = Solution::map(ch);
            if num >= max {
                max = num;
                acc + num
            } else {
                acc - num
            }
        })
    }
}

mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
