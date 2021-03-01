pub struct Solution;
use cauly_rust_leetcode_utils::define_dp;
use cauly_rust_leetcode_utils::dp::*;

struct State {
    maxi: usize,
}
define_dp!(State, maxi:30000);

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut dp = DP::new::<State>(nums[0]);
        let mut result = nums[0];
        for i in 1..nums.len() {
            *dp.get_mut(&State { maxi: i }) =
                std::cmp::max(nums[i], nums[i] + dp.get(&State { maxi: i - 1 }));
            result = std::cmp::max(dp.get(&State { maxi: i }), result);
        }
        result
    }
}
