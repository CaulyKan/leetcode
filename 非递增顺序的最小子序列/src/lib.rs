impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted = nums.clone();
        let mut sum1: i32 = sorted.iter().sum();
        let mut sum2 = 0;
        sorted.sort();
        let mut result = vec![];

        while sorted.len() > 0 {
            let num = sorted.pop().unwrap();
            result.push(num);
            sum1 -= num;
            sum2 += num;
            if sum2 > sum1 {
                break;
            }
        }

        result
    }
}

pub struct Solution;
