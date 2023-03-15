struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1: Vec<_> = num1.bytes().map(|b| b - b'0').rev().collect();
        let num2: Vec<_> = num2.bytes().map(|b| b - b'0').rev().collect();
        let num1 = &num1;
        let num2 = &num2;

        let mut res = vec![0];
        let mut zero_num = 0;
        for &a in num1.iter() {
            let product = Solution::mul(a, num2, zero_num);
            res = Solution::add(&res, &product);
            zero_num += 1;
        }

        String::from_utf8(
            res.into_iter()
                .rev()
                .skip_while(|&b| b == 0)
                .map(|b| b + b'0')
                .collect(),
        )
        .unwrap()
    }

    fn mul(num1: u8, num2: &[u8], zero_num: usize) -> Vec<u8> {
        let mut res = vec![0; zero_num];
        let mut carry = 0;

        for &num in num2 {
            let sum = num * num1 + carry;
            let q = sum / 10;
            let r = sum % 10;
            carry = q;
            res.push(r);
        }

        if carry != 0 {
            res.push(carry);
        }

        res
    }

    fn add(num1: &[u8], num2: &[u8]) -> Vec<u8> {
        let long;
        let short;
        if num1.len() > num2.len() {
            long = num1;
            short = num2;
        } else {
            long = num2;
            short = num1;
        }

        let mut res = vec![];
        let mut carry = 0;
        for (a, b) in long.iter().cloned().zip(short.iter().cloned()) {
            let sum = a + b + carry;
            let q = sum / 10;
            let r = sum % 10;
            carry = q;
            res.push(r);
        }

        for &a in &long[short.len()..] {
            let sum = a + carry;
            let q = sum / 10;
            let r = sum % 10;
            carry = q;
            res.push(r);
        }

        if carry != 0 {
            res.push(carry);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let res = Solution::mul(2, &[5, 5], 0);
        println!("{res:?}");
    }

    #[test]
    fn test_add() {
        let res = Solution::add(&[6, 9], &[4]);
        println!("{res:?}");
    }

    #[test]
    fn test_case() {
        let res = Solution::multiply("123".into(), "456".into());
        println!("{res}");
    }
}
