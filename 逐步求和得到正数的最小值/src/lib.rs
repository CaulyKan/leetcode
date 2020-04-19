impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min = i32::max_value();
        for n in nums {
            sum += n;
            min = std::cmp::min(min, sum);
        }

        if min >= 0 {
            1
        } else {
            -min + 1
        }
    }
}

pub struct Solution;
