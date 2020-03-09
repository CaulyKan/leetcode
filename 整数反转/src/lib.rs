/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        x.signum() * 
                x.abs()
                 .to_string()
                 .chars()
                 .rev()
                 .collect::<String>()
                 .parse::<i32>()
                 .unwrap_or(0)
    }
}

// @lc code=end

