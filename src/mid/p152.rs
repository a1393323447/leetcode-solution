struct Solution;

impl Solution {
    // 动态规划 + 分类讨论
    // 如果当前是正数, 那么就希望能够得到前面子数组的最大乘积, 设为 fmax
    // 如果当前是负数, 那么就希望能够得到前面子数组的最小乘积, 设为 fmin
    // 则需要维护两个 dp 数组, 其递推关系为:
    // fmax(i) = max{ n[i] * fmax[i - 1], n[i] * fmin[i - 1], n[i] }
    // fmin(i) = min{ n[i] * fmax[i - 1], n[i] * fmin[i - 1], n[i] }
    //
    // 又因为只用到了状态 i, i-1 所以可以用优化空间
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n0 = nums[0];

        let mut res = n0;

        let mut fmax = n0;
        let mut fmin = n0;

        for i in 1..nums.len() {
            let ni = nums[i];
            let mx = fmax;
            let mn = fmin;

            if ni < 0 {
                fmax = ni.max(mn * ni);
                fmin = ni.min(mx * ni);
            } else {
                fmax = ni.max(mx * ni);
                fmin = ni.min(mn * ni);
            }

            res = res.max(fmax);
        }

        res
    }
}
