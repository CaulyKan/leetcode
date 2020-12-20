pub struct Solution;
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut result = 0;
        for i in n.chars() {
            let num = i.to_string().parse().unwrap();
            result = std::cmp::max(result, num);
        }
        result
    }
}
