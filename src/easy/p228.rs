struct Solution;

use std::fmt::Display;

struct Interval {
    start: Option<i32>,
    end: Option<i32>,
}

impl Interval {
    fn new() -> Interval {
        Interval {
            start: None,
            end: None,
        }
    }

    fn set_start(&mut self, start: i32) {
        let _ = self.start.insert(start);
    }

    fn set_end(&mut self, end: i32) {
        let _ = self.end.insert(end);
    }

    fn unset(&mut self) {
        self.start = None;
        self.end = None;
    }

    fn have_start(&self) -> bool {
        self.start.is_some()
    }

    fn have_end(&self) -> bool {
        self.end.is_some()
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Interval { start, end } = self;

        match (start, end) {
            (Some(s), Some(e)) => {
                if *s == *e {
                    write!(f, "{s}")
                } else {
                    write!(f, "{s}->{e}")
                }
            }
            _ => std::fmt::Result::Err(std::fmt::Error),
        }
    }
}

impl Solution {
    fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }
        let mut res = vec![];
        let mut interval = Interval::new();
        let mut prev_num = nums[0];
        let last = *nums.last().unwrap();

        for num in nums {
            if !interval.have_start() {
                interval.set_start(prev_num);
            } else if num > 1 + prev_num {
                // 防治溢出
                interval.set_end(prev_num);
                res.push(interval.to_string());
                interval.unset();
                interval.set_start(num);
            }

            prev_num = num;
        }

        if interval.have_start() {
            interval.set_end(last);
            res.push(interval.to_string());
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2".to_string(), "4->5".into(), "7".into()]
        );
    }
}
