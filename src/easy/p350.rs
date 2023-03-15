use std::{
    collections::HashMap,
    ops::{AddAssign, SubAssign},
};

struct Solution;

impl Solution {
    // Hash
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut table = HashMap::new();

        for num in nums1 {
            table.entry(num).or_insert(0).add_assign(1);
        }

        for num in nums2 {
            if let Some(value) = table.get_mut(&num) {
                if *value > 0 {
                    value.sub_assign(1);
                    res.push(num);
                }
            }
        }

        res
    }

    // sort
    pub fn intersect2(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();

        let mut res = vec![];
        let mut p1 = 0;
        let mut p2 = 0;
        let n1 = nums1.len();
        let n2 = nums2.len();
        while p1 < n1 && p2 < n2 {
            let num1 = nums1[p1];
            let num2 = nums2[p2];
            match num1.cmp(&num2) {
                std::cmp::Ordering::Less => p1 += 1,
                std::cmp::Ordering::Equal => {
                    res.push(num1);
                    p1 += 1;
                    p2 += 1;
                }
                std::cmp::Ordering::Greater => p2 += 1,
            }
        }

        res
    }
}
