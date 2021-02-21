pub struct Solution;
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; boxes.len()];
        let boxes: Vec<char> = boxes.chars().collect();

        for i in 0..boxes.len() {
            for j in 0..boxes.len() {
                if i != j {
                    result[i] += (i as i32 - j as i32).abs() * if boxes[j] == '0' { 0 } else { 1 };
                }
            }
        }

        result
    }
}
