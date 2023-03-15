struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().unwrap().len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::length_of_last_word("Hello World".into()), 5);
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".into()),
            4
        );
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".into()),
            6
        );
    }
}
