pub struct Solution;
use cauly_rust_leetcode_utils::union_find::*;
impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        let mut uf = UnionFind::from_iter(nums);
    }
}
