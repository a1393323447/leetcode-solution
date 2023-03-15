struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s = s.into_bytes();
        let mut t = t.into_bytes();

        s.sort();
        t.sort();

        s == t
    }

    // hash
    pub fn is_anagram2(s: String, t: String) -> bool {
        let mut tb = vec![0; 256];
        s.bytes().for_each(|b| tb[b as usize] += 1);
        t.bytes().for_each(|b| tb[b as usize] -= 1);

        tb.iter().all(|&n| n == 0)
    }
}
