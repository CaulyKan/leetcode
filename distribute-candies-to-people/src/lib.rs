pub struct Solution;

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let n = num_people;
        for k in 0..1000000 {
            if ((k + 1) * (k + 1) * n * n + (k + 1) * n) / 2 > candies {
                let mut remain = candies - (k * k * n * n + k * n) / 2;
                let mut result = Vec::new();
                for i in 1..n + 1 {
                    let base = k * i + k * (k - 1) * n / 2;
                    if remain >= n * k + i {
                        result.push(base + n * k + i);
                        remain -= n * k + i;
                    } else {
                        result.push(base + remain);
                        remain = 0;
                    }
                }
                return result;
            }
        }
        panic!();
    }
}
