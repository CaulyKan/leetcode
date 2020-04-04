impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let n = satisfaction.len();
        let mut s = satisfaction.clone();
        s.sort();
        let mut max = i32::min_value();

        for i in (0..n + 1).rev() {
            let mut k = i as i32;
            let mut sum = 0;
            for num in s.iter().rev() {
                sum += *num * k;
                k -= 1;
                if k <= 0 {
                    break;
                }
            }

            println!("{} {}", i, sum);

            max = std::cmp::max(max, sum);
        }

        max
    }
}

pub struct Solution;
