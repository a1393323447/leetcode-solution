struct Solution;

impl Solution {
    // 中心扩散法: 枚举回文中心
    // 回文中心有两种: x...cc...x  x...c...x
    // O(N)
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes(); // s 只由英文和数字组成
        let len = bytes.len();
        if len < 2 {
            return s;
        }
        // (start, end]
        let mut max_len = 0;
        let mut start = 0;
        let mut end = 1;
        for i in 1..len {
            let (odd_start, odd_len) = Solution::expend_around_center(bytes, i, i);
            let (even_start, even_len) = Solution::expend_around_center(bytes, i - 1, i);
            if odd_len > even_len && odd_len > max_len {
                start = odd_start;
                end = odd_start + odd_len;
                max_len = odd_len;
            } else if even_len > max_len {
                start = even_start;
                end = even_start + even_len;
                max_len = even_len;
            }
        }

        unsafe { std::str::from_utf8_unchecked(&bytes[start..end]).into() }
    }
    // 通过中心枚举回文串
    // 回文中心有两种: x...c1c2...x  x...c...x (c1 == c2)
    // 返回枚举得到的最长回文字串的起始坐标和长度(byte 偏移)
    fn expend_around_center(s: &[u8], c1: usize, c2: usize) -> (usize, usize) {
        let len = s.len();
        let mut start = c1;
        let mut end = c2;

        // 沿中心扩散
        while (start as isize) >= 0 && end < len && s[start] == s[end] {
            (start, _) = start.overflowing_sub(1);
            end += 1;
        }

        (start + 1, end - start - 1)
    }

    // 动态规划解法
    // dp[i][j] 表示 s[i..=j] 是否为回文子串
    // 递推式: dp[i][j] = dp[i + 1][j - 1] && s[i] == s[j]
    // 边界条件: dp[i][i] = true (只有一个字符的明显时回文串)
    //          dp[i][i+1] = s[i] == s[j] (只有两个字符时也是边界条件之一, 因为递推式的判断要求子串至少有两个字符, 也因为有奇偶中心)
    // 分析: dp[i][j] = dp[i + 1][j - 1] 这个递推式 i 依赖比它大的, 而 j 依赖比它小的,
    //       所以递推顺序应该为:
    //       i: 递减
    //       j: 递增
    pub fn longest_palindrome2(s: String) -> String {
        let bytes = s.as_bytes(); // s 只由英文和数字组成
        let len = bytes.len();

        if len < 2 {
            return s;
        }

        let mut dp = vec![vec![false; len]; len];
        // 边界条件
        for i in 0..len {
            dp[i][i] = true;
        }

        let mut max_len = 1;
        let mut start = 0;
        // 为了缓存友好可以设置为 dp[j][i], 但现在不做这个优化
        for j in 1..len {
            for i in 0..j {
                if bytes[i] != bytes[j] {
                    dp[i][j] = false;
                } else {
                    if j - i < 3 {
                        dp[i][j] = true;
                    } else {
                        dp[i][j] = dp[i + 1][j - 1];
                    }
                }

                let cur_len = j - i + 1;
                if dp[i][j] && max_len < cur_len {
                    max_len = cur_len;
                    start = i;
                }
            }
        }
        let end = start + len;
        unsafe { std::str::from_utf8_unchecked(&bytes[start..end]).into() }
    }

    // 还有一种 马拉车 算法, 有待补充 O(N)
    // https://leetcode.cn/problems/longest-palindromic-substring/solutions/255195/zui-chang-hui-wen-zi-chuan-by-leetcode-solution/
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::longest_palindrome("bb".into()), "bb");
    }
}
