pub struct Solution;
use cauly_rust_leetcode_utils::binary_search::*;
impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut result = 0;
        let n2 = nums2.len();
        let bs = BinarySearch::from_sorted(nums2, |x| -x);
        for i in 0..nums1.len() {
            let idx = bs.find_first_index_larger_than(&nums1[i]).unwrap_or(n2 - 1);
            println!("{} {}", i, idx);
            if idx != 999999 && idx >= i && idx - i > result {
                result = idx - i;
            }
        }

        result as i32
    }
}
