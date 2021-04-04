pub struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let mut i = k as usize;
        let mut j = k as usize;
        let mut result = i32::min_value();
        let mut min = nums[k as usize];
        loop {
            while i >= 1 && nums[i - 1] >= min {
                i -= 1;
            }
            while j + 1 < nums.len() && nums[j + 1] >= min {
                j += 1;
            }
            result = std::cmp::max(result, min * (j as i32 - i as i32 + 1));
            if i == 0 && j == nums.len() - 1 {
                break;
            } else if i == 0 && j < nums.len() - 1 {
                min = nums[j + 1];
            } else if j == nums.len() - 1 && i > 0 {
                min = nums[i - 1];
            } else {
                if nums[i - 1] < nums[j + 1] {
                    min = nums[j + 1];
                } else {
                    min = nums[i - 1];
                }
            }
        }

        result
    }
}
