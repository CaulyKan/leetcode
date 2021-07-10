pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut result = 0;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                continue;
            }
            result += nums[i - 1] + 1 - nums[i];
            nums[i] = nums[i - 1] + 1;
        }
        result
    }
}
