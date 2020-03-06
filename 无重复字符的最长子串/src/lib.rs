/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = 0;
        let mut right = 1;
        let mut max = 1;

        if s.len() == 0 {
            return 0;
        }

        while right < s.len() {
            for i in left..right {
                if s.chars().nth(i).unwrap() == s.chars().nth(right).unwrap() {
                    left = i + 1;
                }
            }
            right += 1;
            if (right - left) > max {
                max = right - left;
            }
        }
        max as i32
    }
}
// @lc code=end
pub struct Solution;
