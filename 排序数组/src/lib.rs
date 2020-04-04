/*
 * @lc app=leetcode.cn id=912 lang=rust
 *
 * [912] 排序数组
 */

// @lc code=start
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            nums
        } else {
            let n = nums[0];
            Solution::sort_array(
                nums.iter()
                    .skip(1)
                    .filter(|&&x| x < n)
                    .map(|&x| x)
                    .collect(),
            )
            .iter()
            .chain(vec![&n])
            .chain(
                Solution::sort_array(
                    nums.iter()
                        .skip(1)
                        .filter(|&&x| x >= n)
                        .map(|&x| x)
                        .collect(),
                )
                .iter(),
            )
            .map(|&x| x)
            .collect()
        }
    }
}
// @lc code=end

pub struct Solution;
