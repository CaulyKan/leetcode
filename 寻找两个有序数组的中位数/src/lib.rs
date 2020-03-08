/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个有序数组的中位数
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut l1 = 0;
        let mut l2 = 0;
        let is_odd = (nums1.len() + nums2.len()) % 2 != 0;
        let next = |l1: &mut usize, l2: &mut usize| -> i32 {
            let mut l = -1;
            if nums1.get(*l1).unwrap_or(&i32::max_value())
                <= nums2.get(*l2).unwrap_or(&i32::max_value())
            {
                l = nums1[*l1];
                *l1 += 1;
            } else {
                l = nums2[*l2];
                *l2 += 1;
            }
            l
        };
        let mut last = 0;
        while l1 + l2 < (nums1.len() + nums2.len()) / 2 {
            last = next(&mut l1, &mut l2);
        }
        if is_odd {
            next(&mut l1, &mut l2) as f64
        } else {
            (next(&mut l1, &mut l2) + last) as f64 / 2 as f64
        }
    }
}
// @lc code=end
