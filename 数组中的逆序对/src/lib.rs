impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        if nums.len() == 0 {
            return 0;
        }

        for i in 0..(nums.len() - 1) {
            let n = nums[i];
            for j in (i + 1)..nums.len() {
                if nums[j] < n {
                    result += 1;
                }
            }
        }

        result
    }
}

pub struct Solution;
