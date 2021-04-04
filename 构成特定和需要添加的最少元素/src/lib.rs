pub struct Solution;
impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        let mut current: i64 = 0;
        for i in nums {
            current += i as i64;
        }
        ((goal as i64 - current).abs() / limit as i64
            + if (goal as i64 - current).abs() % limit as i64 == 0 {
                0
            } else {
                1
            }) as i32
    }
}
