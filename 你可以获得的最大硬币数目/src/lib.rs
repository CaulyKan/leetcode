pub struct Solution;
impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let mut p = piles.clone();
        p.sort();
        p.reverse();
        let n = p.len() / 3;
        let mut result = 0;
        for i in 0..n {
            result += p[i * 2 + 1];
        }

        result
    }
}
