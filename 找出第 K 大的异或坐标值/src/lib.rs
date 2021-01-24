pub struct Solution;

impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut mat2 = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut pq = std::collections::BinaryHeap::new();

        for j in 0..matrix[0].len() {
            for i in 0..matrix.len() {
                if i == 0 && j == 0 {
                    mat2[0][0] = matrix[0][0];
                } else if i == 0 {
                    mat2[i][j] = matrix[i][j] ^ mat2[i][j - 1];
                } else if j == 0 {
                    mat2[i][j] = matrix[i][j] ^ mat2[i - 1][j];
                } else {
                    mat2[i][j] =
                        matrix[i][j] ^ mat2[i - 1][j] ^ mat2[i][j - 1] ^ mat2[i - 1][j - 1];
                }
                pq.push(mat2[i][j]);
            }
        }

        for _ in 0..k - 1 {
            pq.pop();
        }

        *pq.peek().unwrap()
    }
}
