extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let mut result = Solution::min_distance(String::from("intention"), String::from("execution"));
    assert_eq!(5, result);
}
