struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        for num in digits.iter_mut().rev() {
            *num += carry;
            carry = *num / 10;
            *num %= 10;
        }

        if carry != 0 {
            digits.insert(0, carry);
        }

        digits
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }
}
