pub struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut result = Vec::new();

        for r in 0..row {
            let mut min = i32::max_value();
            let mut min_col = 0;
            for c in 0..col {
                if min > matrix[r][c] {
                    min = matrix[r][c];
                    min_col = c;
                }
            }
            let mut max = i32::min_value();
            for r1 in 0..row {
                max = std::cmp::max(max, matrix[r1][min_col]);
            }
            if max == min {
                result.push(max);
            }
        }

        result
    }
}
