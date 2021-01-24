pub struct Solution;
use std::ops::BitXor;
impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result = vec![];

        result.push(first);

        for i in 0..encoded.len() {
            result.push(result[i] ^ encoded[i]);
        }

        result
    }
}
