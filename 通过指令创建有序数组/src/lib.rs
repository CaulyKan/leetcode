pub struct Solution;

use cauly_rust_leetcode_utils::binary_search::*;

impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut result: i64 = 0;
        let mut bs = BinarySearch::new(|x| *x);
        for i in instructions {
            let l = bs.how_many_values_smaller_than(&i);
            let r = bs.how_many_values_larger_than(&i);
            result += std::cmp::min(l, r) as i64;
            bs.insert(i);
        }
        (result % 1000000007) as i32
    }
}
