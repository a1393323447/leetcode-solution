use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut res = String::new();
        let mut set = HashMap::new();

        // 防止溢出
        let numerator = numerator as i64;
        let denominator = denominator as i64;

        let mut idx = 0;

        // 计算整数部分
        let q = numerator / denominator;
        let mut r = numerator % denominator;

        if q == 0 {
            if (numerator < 0) ^ (denominator < 0) && r != 0 {
                res.push('-');
                idx += 1;
            }
            res.push('0');
            idx += 1;
        } else {
            let q_str = q.to_string();
            res.push_str(&q_str);
            idx += q_str.len();
        }

        if r == 0 {
            // 整除
            return res;
        } else {
            // 小数
            res.push('.');
            idx += 1;
            set.insert(r, idx);
        }

        // 计算小数部分
        r = r.abs() * 10;
        let denominator = denominator.abs();

        loop {
            let q = r / denominator;
            r = r % denominator;

            debug_assert!(q < 10);

            res.push((b'0' + q as u8) as char);
            idx += 1;

            if r == 0 {
                break;
            }

            if let Some(&p_idx) = set.get(&r) {
                res.insert(p_idx, '(');
                res.push(')');
                break;
            } else {
                set.insert(r, idx);
            }

            r *= 10;
        }

        res
    }
}
