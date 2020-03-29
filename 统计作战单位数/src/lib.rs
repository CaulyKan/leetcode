impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..rating.len() {
            for j in i..rating.len() {
                if rating[j] > rating[i] {
                    for k in j..rating.len() {
                        if rating[k] > rating[j] {
                            result += 1;
                        }
                    }
                }
            }
        }
        for i in 0..rating.len() {
            for j in i + 1..rating.len() {
                if rating[j] < rating[i] {
                    for k in j + 1..rating.len() {
                        if rating[k] < rating[j] {
                            result += 1;
                        }
                    }
                }
            }
        }
        result
    }
}

pub struct Solution;
