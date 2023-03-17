struct Solution;

impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, mut queries: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let m = queries.len();
        let mut answer = vec![n as i32; m];

        let nums = {
            nums.sort();
            nums
        };
        let queries = {
            let mut q: Vec<_> = queries.into_iter().enumerate().collect();
            q.sort_by(|a, b| a.1.cmp(&b.1));

            q
        };

        let mut pre_len = 0;
        let mut pre_sum = 0;
        let mut cur_num_idx = 0;
        for (answer_idx, require_sum) in queries {
            let mut remain = require_sum - pre_sum;
            let mut cur_len = pre_len;
            let mut cur_sum = pre_sum;
            while remain >= nums[cur_num_idx] {
                remain -= nums[cur_num_idx];
                cur_sum += nums[cur_num_idx];
                cur_len += 1;
                cur_num_idx += 1;

                if cur_num_idx == n {
                    return answer;
                }
            }
            answer[answer_idx] = cur_len as i32;
            pre_len = cur_len;
            pre_sum = cur_sum;
        }

        answer
    }

    pub fn answer_queries2(mut nums: Vec<i32>, mut queries: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        nums.iter_mut().fold(0, |s, x| {
            *x += s;
            *x
        });
        queries
            .iter_mut()
            .for_each(|x| *x = nums.partition_point(|y| y <= x) as i32);
        queries
    }
}
