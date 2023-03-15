struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;
        let mut idx1 = m - 1;
        let mut idx2 = n - 1;
        let mut idx = m + n - 1;

        while (idx1 as isize) >= 0 && (idx2 as isize) >= 0 {
            if nums1[idx1] > nums2[idx2] {
                nums1[idx] = nums1[idx1];
                idx1 = (idx1 as isize - 1) as _;
            } else {
                nums1[idx] = nums2[idx2];
                idx2 = (idx2 as isize - 1) as _;
            }
            idx -= 1;
        }

        if (idx2 as isize) >= 0 {
            nums1[..=idx2].copy_from_slice(&nums2[..=idx2]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
