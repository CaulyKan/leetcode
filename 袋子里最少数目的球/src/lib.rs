pub struct Solution;
impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let max_count = nums.len() as i32 + max_operations;
        let mut sum: f64 = 0.0;
        for i in &nums {
            sum += *i as f64;
        }

        let mut result = (sum / max_count as f64).floor() as i32
            + if sum % max_count as f64 == 0.0 { 0 } else { 1 };
        println!("{} {}", result, max_count);
        loop {
            let mut count = 0.0;
            for i in &nums {
                let mut temp =
                    (*i as f64 / result as f64).floor() + if *i % result == 0 { 0.0 } else { 1.0 };
                count += temp;
                if count > max_count as f64 {
                    result += 1;
                    break;
                }
            }
            if count <= max_count as f64 {
                break result;
            }
        }
    }
}
