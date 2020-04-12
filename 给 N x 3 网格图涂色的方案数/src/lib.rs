impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut a: i64 = 6;
        let mut b: i64 = 6;
        let nn: i64 = 1000000007;
        for _ in 1..n {
            let c = a * 3 + b * 2;
            let d = a * 2 + b * 2;
            a = c % nn;
            b = d % nn;
        }
        ((a + b) % nn) as i32
    }
}

pub struct Solution;
