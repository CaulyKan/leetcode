pub struct Solution;
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut sum = 0;

        let mut exists = vec![false; 10001];

        let mut l = 0;
        let mut r = 0;

        while r < nums.len() {
            let current = nums[r] as usize;
            if exists[current] {
                let removing = nums[l] as usize;
                sum -= removing;
                exists[removing] = false;
                l += 1;
            } else {
                sum += current;
                result = std::cmp::max(sum, result);
                exists[current] = true;
                r += 1;
            }
        }

        result as i32
    }
}
