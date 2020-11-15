pub struct Solution;
impl Solution {
    pub fn keyboard(k: i32, n: i32) -> i32 {
        use std::cmp;

        /// Calculate the number of combinations when choosing
        /// k elements from n elements without replacement, multiplied by a scaling factor.
        pub fn scaled_combinations(n: u64, k: u64, scale: f64) -> f64 {
            if k > n {
                0.0
            } else {
                let mut comb = scale;
                for j in 0..cmp::min(k, n - k) {
                    comb /= (j + 1) as f64;
                    comb *= (n - j) as f64;
                }
                comb
            }
        }

        /// Calculate the number of combinations when choosing
        /// k elements from n elements without replacement.
        /// This is also known as n over k, or the binomial coefficient.
        pub fn combinations(n: u64, k: u64) -> f64 {
            scaled_combinations(n, k, 1.0)
        }

        let k = k as f64;
        let n = n as f64;
        let mut result = (26 as f64).powf(n);
        println!("  {}", result);

        for i in k as usize + 1..n as usize + 1 {
            let a = 26 as f64
                * combinations(n as u64, i as u64)
                * combinations(k as u64 * 25 as u64, n as u64 - i as u64);
            result -= a;
            println!("{} {}", i, a);
        }

        result = result % 1000000007 as f64;
        return result as i32;
    }
    pub fn keyboard2(k: i32, n: i32) -> i32 {
        let mut p: Vec<((i32, i32, i32, i32, i32), i64)> = vec![];
        let mut result: i64 = 0;

        if k == 1 {
            p.push(((26, 0, 0, 0, 0), 1));
        } else if k == 2 {
            p.push(((0, 26, 0, 0, 0), 1));
        } else if k == 3 {
            p.push(((0, 0, 26, 0, 0), 1));
        } else if k == 4 {
            p.push(((0, 0, 0, 26, 0), 1));
        } else if k == 5 {
            p.push(((0, 0, 0, 0, 26), 1));
        }

        for _ in 0..n {
            let mut p1: Vec<((i32, i32, i32, i32, i32), i64)> = vec![];
            for (q, k) in p {
                if q.0 != 0 {
                    p1.push(((q.0 - 1, q.1, q.2, q.3, q.4), k * q.0 as i64 % 1000000007));
                }
                if q.1 != 0 {
                    p1.push((
                        (q.0 + 1, q.1 - 1, q.2, q.3, q.4),
                        k * q.1 as i64 % 1000000007,
                    ));
                }
                if q.2 != 0 {
                    p1.push((
                        (q.0, q.1 + 1, q.2 - 1, q.3, q.4),
                        k * q.2 as i64 % 1000000007,
                    ));
                }
                if q.3 != 0 {
                    p1.push((
                        (q.0, q.1, q.2 + 1, q.3 - 1, q.4),
                        k * q.3 as i64 % 1000000007,
                    ));
                }
                if q.4 != 0 {
                    p1.push((
                        (q.0, q.1, q.2, q.3 + 1, q.4 - 1),
                        k * q.4 as i64 % 1000000007,
                    ));
                }
                print!("{:?} {}", q, k);
            }
            p = p1;
            println!();
        }

        for (_, k) in p {
            result += k;
        }
        if result > 1000000007 {
            result = result % 1000000007;
        }

        result as i32
    }
}
