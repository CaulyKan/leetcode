impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in coins {
            result += (i + 1) / 2;
        }
        result
    }
}

pub struct Solution;
