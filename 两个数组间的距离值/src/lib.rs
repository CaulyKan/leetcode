pub struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut result = 0;
        for i in arr1 {
            if arr2.iter().all(|x| (x - i).abs() > d) {
                result += 1;
            }
        }
        result
    }
}
