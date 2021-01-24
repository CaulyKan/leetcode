pub struct Solution;
use std::cmp::*;
impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;
        let mut colMat = vec![Vec::new(); matrix[0].len()];
        let mut zeroCount = 0;
        for row in &matrix {
            for i in 0..row.len() {
                colMat[i].push(row[i]);
                if row[i] == 0 {
                    zeroCount += 1;
                }
            }
        }

        if zeroCount == 0 {
            return matrix.len() as i32 * matrix[0].len() as i32;
        }

        for row in 0..matrix.len() {
            let mut ones: Vec<&Vec<i32>> = colMat.iter().filter(|x| x[row] == 1).collect();
            result = max(result, ones.len() as i32);
            for grow in row + 1..matrix.len() {
                ones = ones.into_iter().filter(|x| x[grow] == 1).collect();
                if ones.len() == 0 {
                    break;
                }
                result = max(result, ones.len() as i32 * (grow - row + 1) as i32);
            }
        }
        result
    }
}
