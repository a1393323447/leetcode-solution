struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1 = version1.split('.').map(|s| s.parse::<i32>().unwrap());
        let mut v2 = version2.split('.').map(|s| s.parse::<i32>().unwrap());

        loop {
            match (v1.next(), v2.next()) {
                (None, None) => return 0,
                (None, Some(v)) => {
                    if v > 0 || v2.any(|v| v > 0) {
                        return -1;
                    } else {
                        return 0;
                    }
                }
                (Some(v), None) => {
                    if v > 0 || v1.any(|v| v > 0) {
                        return 1;
                    } else {
                        return 0;
                    }
                }
                (Some(vi1), Some(vi2)) => match vi1.cmp(&vi2) {
                    std::cmp::Ordering::Less => return -1,
                    std::cmp::Ordering::Equal => continue,
                    std::cmp::Ordering::Greater => return 1,
                },
            }
        }
    }
}
