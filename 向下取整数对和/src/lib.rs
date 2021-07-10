pub struct Solution;
use cauly_rust_leetcode_utils::segment_tree::SegmentTree;
impl Solution {
    pub fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
        let mut l = [0 as usize; 100001];
        for i in nums {
            l[i as usize] += 1;
        }
        let st = SegmentTree::from_simple(&l, Box::new(|&x, &y| x + y));

        let mut result: u64 = 0;
        for n in 0..100001 {
            result = (result + l[n] as u64 * l[n] as u64) % 1000000007;

            if l[n] > 0 {
                let mut v = 1;
                while v * n < 100001 {
                    let tt = st.query(n * v + 1, std::cmp::min(100000, n * (v + 1) - 1), 0);
                    //if tt > 0 {
                    println!("{}: {}, {}: {}", n, n * v + 1, n * (v + 1) - 1, tt);
                    //}
                    result = (result + tt as u64 * v as u64) % 1000000007;
                    v += 1;
                }
            }
        }

        result as i32
    }
}
