struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n).map(Solution::to_fizz_buzz).collect()
    }

    fn to_fizz_buzz(n: i32) -> String {
        if n % 3 == 0 && n % 5 == 0 {
            "FizzBuzz".to_string()
        } else if n % 3 == 0 {
            "Fizz".to_string()
        } else if n % 5 == 0 {
            "Buzz".to_string()
        } else {
            n.to_string()
        }
    }
}
