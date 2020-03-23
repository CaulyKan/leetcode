/*
 * @lc app=leetcode.cn id=300 lang=rust
 *
 * [300] 最长上升子序列
 */

// @lc code=start
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::BTreeMap::new();
        let mut global_max = 0;

        for num in nums {
            let mut max = 0;
            for (i, len) in map.iter() {
                if *i < num {
                    max = std::cmp::max(max, *len);
                } else {
                    break;
                }
            }
            map.insert(num, max + 1);
            global_max = std::cmp::max(global_max, max + 1);
        }

        global_max
    }
}
// @lc code=end

pub struct Solution;
