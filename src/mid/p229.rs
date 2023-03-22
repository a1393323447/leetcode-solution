use std::collections::HashSet;

struct Solution;

#[derive(Debug, Clone, Copy)]
struct Candidate {
    no: i32,
    vote: i32,
}

impl Candidate {
    fn new(no: i32, vote: i32) -> Self {
        Self { no, vote }
    }

    fn has_vote(&self) -> bool {
        self.vote > 0
    }

    fn update(&mut self, no: i32) -> bool {
        if self.no == no {
            self.vote += 1;
            return true;
        } else {
            self.vote -= 1;
            return false;
        }
    }
}

impl Solution {
    // 摩尔投票法
    // https://leetcode.cn/problems/majority-element-ii/solutions/123170/liang-fu-dong-hua-yan-shi-mo-er-tou-piao-fa-zui-zh/
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let k = 2;
        let mut candidates: Vec<Candidate> = vec![];
        for &no in nums.iter() {
            if candidates.len() < k {
                if let Some(c) = candidates.iter_mut().find(|c| c.no == no) {
                    c.vote += 1;
                } else {
                    candidates.push(Candidate::new(no, 1));
                }
            } else {
                for (idx, candidate) in candidates.iter_mut().enumerate() {
                    if candidate.update(no) {
                        candidates[..idx].iter_mut().for_each(|c| { c.vote += 1; });
                        break;
                    }
                }
                candidates.retain(|c| c.has_vote());
            }
        }

        let set: HashSet<_> = candidates.into_iter().map(|c| c.no).collect();
        let mut candidates: Vec<_> = set.into_iter().map(|no| Candidate::new(no, 0)).collect();

        for no in nums {
            for candidate in candidates.iter_mut() {
                if candidate.no == no {
                    candidate.vote += 1;
                }
            }
        }

        candidates.into_iter().filter(|c| c.vote > (n as i32 / 3)).map(|c| c.no).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        Solution::majority_element(vec![1,1,2,3,3]);
    }
}