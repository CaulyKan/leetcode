pub struct Solution;

use std::cmp::*;
use std::collections::BinaryHeap;
impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let mut pq = BinaryHeap::new();
        let n = arr.len() as i32;
        for i in arr {
            pq.push(Reverse(i));
        }

        let mut last = 0;
        for i in (1..n + 1) {
            let cur = pq.pop().unwrap().0;
            let v = min(min(cur, i), last + 1);
            last = v;
        }

        last
    }
}
