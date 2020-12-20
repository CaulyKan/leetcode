pub struct Solution;

use std::cmp::*;
#[derive(Eq)]
struct P {
    val: i32,
    pos: i32,
}
impl P {
    pub fn new(val: i32, pos: i32) -> Self {
        Self { pos, val }
    }
}
impl Ord for P {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}
impl PartialOrd for P {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for P {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.pos == other.pos
    }
}
impl Solution {
    // pub fn max_result2(nums: Vec<i32>, k: i32) -> i32 {
    //     let mut dp = vec![0; nums.len()];
    //     dp[0] = nums[0];
    //     for i in 1..nums.len() {
    //         let mut mx = i32::min_value();
    //         let l = max(0, i as i32 - k) as usize;
    //         for j in l..i {
    //             mx = max(mx, dp[j]);
    //         }
    //         dp[i] = mx + nums[i];
    //         println!("{:?}", dp);
    //     }

    //     dp[nums.len() - 1]
    // }
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = std::collections::BinaryHeap::new();
        dp.push(P::new(nums[0], 0));
        let mut last = nums[0];
        for i in 1..nums.len() {
            while dp.peek().unwrap().pos < i as i32 - k {
                dp.pop();
            }
            last = dp.peek().unwrap().val + nums[i];
            dp.push(P::new(last, i as i32));
            //println!("{:?}", dp);
        }

        last
    }
}
