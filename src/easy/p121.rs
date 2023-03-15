struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_prof = 0;

        for price in prices {
            min_price = min_price.min(price);
            let profit = price - min_price;
            max_prof = max_prof.max(profit);
        }

        max_prof
    }
}
