/*
 * @lc app=leetcode.cn id=322 lang=rust
 *
 * [322] 零钱兑换
 */

// @lc code=start
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut result = vec![-1; amount + 1];
        result[0] = 0;
        if amount == 0 {
            return 0;
        }
        for c in &coins {
            if amount == *c as usize {
                return 1;
            }
            if (*c as usize) < amount {
                result[*c as usize] = 1;
            }
        }
        for i in 0..amount + 1 {
            if result[i] == -1 {
                let c = coins
                    .iter()
                    .map(|x| {
                        if i as i32 - *x >= 0 {
                            result[i - *x as usize]
                        } else {
                            -1
                        }
                    })
                    .filter(|x| *x > 0)
                    .min()
                    .unwrap_or(-1);
                if c > 0 {
                    result[i] = c + 1;
                }
            }
        }
        println!("{:?}", result);
        result[amount]
    }
}
// @lc code=end

pub struct Solution;
