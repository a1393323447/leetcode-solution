struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        #[rustfmt::skip]
        let table = vec![
            vec![], vec![], // unused
            vec![b'a', b'b', b'c'],
            vec![b'd', b'e', b'f'],
            vec![b'g', b'h', b'i'],
            vec![b'j', b'k', b'l'],
            vec![b'm', b'n', b'o'],
            vec![b'p', b'q', b'r', b's'],
            vec![b't', b'u', b'v'],
            vec![b'w', b'x', b'y', b'z'],
        ];

        let mut res = vec![];
        let mut cache = String::new();
        let digits = digits.into_bytes();

        Solution::dfs(&digits, &mut cache, &mut res, &table);

        res
    }

    fn dfs(digits: &[u8], cache: &mut String, res: &mut Vec<String>, table: &Vec<Vec<u8>>) {
        if digits.is_empty() {
            res.push(cache.clone());
            return;
        }
        let digit = (digits[0] - b'0') as usize;
        for alp in &table[digit] {
            cache.push(*alp as char);
            Solution::dfs(&digits[1..], cache, res, table);
            cache.pop();
        }
    }
}
