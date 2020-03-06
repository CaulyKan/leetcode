impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for n in (2..target).rev() {
            if n % 2 == 0 {
                if (target as f64 / n as f64).fract() == 0.5 {
                    let mid = target / n;
                    let small = mid - n / 2 + 1;
                    if small > 0 {
                        let seq = (small..n + small).collect();
                        result.push(seq);
                    }
                }
            } else {
                if target % n == 0 {
                    let mid = target / n;
                    let small = mid - n / 2;
                    if small > 0 {
                        let seq = (small..n + small).collect();
                        result.push(seq);
                    }
                }
            }
        }
        result
    }
}

pub struct Solution;
