use cauly_rust_leetcode_utils::segment_tree::*;

pub struct MajorityChecker {
    st: SegmentTree<i32, HashMap<i32, i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
use std::collections::HashMap;
impl MajorityChecker {
    pub fn new(arr: Vec<i32>) -> Self {
        fn aggr_func(x: &HashMap<i32, i32>, y: &HashMap<i32, i32>) -> HashMap<i32, i32> {
            let mut result = x.clone();
            for (&k, &v) in y {
                let v2 = result.entry(k).or_insert(0);
                *v2 += v;
            }
            result
        }
        fn conv_func(x: &i32) -> HashMap<i32, i32> {
            let mut result = HashMap::new();
            result.insert(*x, 1);
            result
        }
        let slice = arr.as_slice();
        MajorityChecker {
            st: SegmentTree::from(slice, Box::new(conv_func), Box::new(aggr_func)),
        }
    }

    pub fn query(&mut self, left: i32, right: i32, threshold: i32) -> i32 {
        for (k, v) in self.st.query(left as usize, right as usize + 1) {
            if v >= threshold {
                return k;
            }
        }
        -1
    }
}
