/*
 * @lc app=leetcode.cn id=169 lang=rust
 *
 * [169] 多数元素
 */

// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut max = 0;
        let mut max_v = 0;
        for num in nums {
            let n = map.entry(num).or_insert(0);
            *n += 1;
            if *n > max {
                max = *n;
                max_v = num;
            }
        }
        max_v
    }
}
// @lc code=end
pub struct Solution;
