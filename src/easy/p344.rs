struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }

    pub fn reverse_string2(s: &mut Vec<char>) {
        let n = s.len();
        for i in 0..(n / 2) {
            (s[i], s[n - i - 1]) = (s[n - i - 1], s[i]);
        }
    }
}
