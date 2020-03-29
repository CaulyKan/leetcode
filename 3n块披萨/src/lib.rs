impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        // let s = slices;
        // let n = s.len();
        // let mut dp = vec![vec![0; n / 3]; n];

        // fn max(a: i32, b: i32, c: i32) -> i32 {
        //     std::cmp::max(a, std::cmp::max(b, c))
        // }
        // fn max4(a: i32, b: i32, c: i32, d: i32) -> i32 {
        //     std::cmp::max(std::cmp::max(a, std::cmp::max(b, c)), d)
        // }
        // fn next_index(i: usize, l: usize, slices: &Vec<i32>) -> usize {
        //     if i + l < slices.len() {
        //         i + l
        //     } else {
        //         i + l - slices.len()
        //     }
        // }
        // fn next(i: usize, l: usize, slices: &Vec<i32>) -> i32 {
        //     slices[next_index(i, l, slices)]
        // }

        // for j in 0..(n / 3) {
        //     if j == 0 {
        //         for i in 0..n {
        //             dp[i][0] = max(next(i, 0, &s), next(i, 1, &s), next(i, 2, &s));
        //         }
        //     } else {
        //         for i in 0..n {
        //             let mut x = max4(
        //                 dp[next_index(i, 3, &s)][j - 1] + next(i, 1, &s),
        //                 dp[next_index(i, 2, &s)][j - 1] + next(i, 0, &s),
        //                 dp[next_index(i, 1, &s)][j - 1] + next(i, j * 3 + 2, &s),
        //                 dp[i][j - 1] + next(i, j * 3 + 1, &s),
        //             );

        //             if j >= 2 {
        //                 for p in 0..(j - 1) {
        //                     let c = dp[next_index(i, 1, &s)][p]
        //                         + dp[next_index(i, 2 + (p + 1) * 3, &s)][j - 2 - p]
        //                         + max(
        //                             next(i, 0, &s),
        //                             next(i, 1 + (p + 1) * 3, &s),
        //                             next(i, j * 3 + 2, &s),
        //                         );
        //                     if c > x {
        //                         x = c;
        //                     }
        //                 }
        //             }

        //             dp[i][j] = x;
        //         }
        //     }
        // }

        // println!("{:?}", dp);

        // let mut result = i32::min_value();
        // for i in 0..n {
        //     result = std::cmp::max(result, dp[i][n / 3 - 1]);
        // }
        // result

        use std::cmp::*;
        fn find_max(v: &[i32]) -> i32 {
            println!("\n{:?}", v);
            let n = v.len();
            let mut max_col = vec![vec![0; (n + 1) / 3]; n];
            max_col[0] = vec![v[0]; (n + 1) / 3];
            max_col[1] = vec![max(v[0], v[1]); (n + 1) / 3];

            for i in 2..n {
                for j in 0..((n + 1) / 3) {
                    if j == 0 {
                        max_col[i][j] = *(&v[0..i + 1]).iter().max().unwrap();
                    } else {
                        max_col[i][j] = max(max_col[i - 1][j], max_col[i - 2][j - 1] + v[i]);
                    }
                }
            }

            for c in &max_col {
                println!("{:?}", c);
            }

            max_col[n - 1][(n + 1) / 3 - 1]
        }

        std::cmp::max(
            find_max(&slices[0..slices.len() - 1]),
            find_max(&slices[1..slices.len()]),
        )
    }
}

pub struct Solution;
