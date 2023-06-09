struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let ia = u128::from_str_radix(&a, 2).unwrap_or(0);
        let ib = u128::from_str_radix(&b, 2).unwrap_or(0);
        format!("{:b}", (ia + ib))
    }
}
