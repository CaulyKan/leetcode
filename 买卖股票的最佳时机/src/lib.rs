/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }

        let mut profit = vec![0; prices.len()];
        let mut min = prices[0];
        for n in 1..prices.len() {
            min = std::cmp::min(min, prices[n]);
            if prices[n] - min > profit[n - 1] {
                profit[n] = prices[n] - min;
            } else {
                profit[n] = profit[n - 1];
            }
        }
        println!("{:?}", profit);
        profit[prices.len() - 1]
    }
}
// @lc code=end

pub struct Solution;
