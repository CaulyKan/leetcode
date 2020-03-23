pub struct Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..(nums.len()) {
            let n = nums[i];
            let idx = index[i] as usize;
            result.insert(idx, n);
        }
        result
    }
}
