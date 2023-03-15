struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        let mut left = 0;
        let mut right = bytes.len() - 1;

        while (left as isize) < (right as isize) {
            // 跳过非数字字母
            while left < right && !bytes[left].is_ascii_alphanumeric() {
                left += 1;
            }
            while left < right && !bytes[right].is_ascii_alphanumeric() {
                right -= 1;
            }
            if (left as isize) >= (right as isize) {
                break;
            }
            let lc = bytes[left].to_ascii_lowercase();
            let rc = bytes[right].to_ascii_lowercase();
            if lc != rc {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".into()),
            true
        );
        assert_eq!(Solution::is_palindrome("a.".into()), true);
    }
}
