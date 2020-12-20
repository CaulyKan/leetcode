extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::min_partitions("27346209830709182346".to_string());
    assert_eq!(9, result);
}
