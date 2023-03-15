struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut si = 0;
        let mut ti = 0;

        while ti < t.len() && si < s.len() {
            if s[si] == t[ti] {
                si += 1;
            }
            ti += 1;
        }

        si == s.len()
    }
}
