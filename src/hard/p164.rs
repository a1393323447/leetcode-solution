struct Solution;

#[derive(Debug, Clone, Copy)]
struct Bucket {
    min: i32,
    max: i32,
}

impl Default for Bucket {
    fn default() -> Self {
        Self { min: -1, max: -1 }
    }
}

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let len = nums.len();
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for i in 0..(len / 2) {
            let j = len - 1 - i;
            if nums[i] < nums[j] {
                min = min.min(nums[i]);
                max = max.max(nums[j]);
            } else {
                min = min.min(nums[j]);
                max = max.max(nums[i]);
            }
        }

        let bucket_size = ((max - min) as usize / (len - 1)).max(1);
        let bucket_cnt = (max - min) as usize / bucket_size + 1;
        let mut buckets = vec![Bucket::default(); bucket_cnt];

        for num in nums {
            let bucket_idx = (num - min) as usize / bucket_size;
            let bucket = &mut buckets[bucket_idx];

            bucket.min = num.min(bucket.min);
            bucket.max = num.max(bucket.max);
        }

        let mut max_gap = 0;
        let mut prev: Option<Bucket> = None;
        for i in 0..bucket_cnt {
            let cbucket = buckets[i];
            if cbucket.min == i32::MAX {
                continue;
            }

            if let Some(pbucket) = prev {
                let gap = cbucket.min - pbucket.max;
                max_gap = max_gap.max(gap);
            }

            prev = Some(cbucket);
        }

        max_gap
    }

    // 逃课版
    pub fn maximum_gap2(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let mut max_gap = i32::MIN;
        for w in nums.windows(2) {
            max_gap = max_gap.max(w[1] - w[0]);
        }

        max_gap
    }
}
