pub struct Solution;
impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        let mut min = i32::max_value();
        let mut max = 0;
        for i in 0..tasks.len() {
            sum += tasks[i][0];
            min = std::cmp::min(min, tasks[i][1] - tasks[i][0]);
            max = std::cmp::max(max, tasks[i][1]);
        }
        std::cmp::max(sum + min, max)
    }
}
