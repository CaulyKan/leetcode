extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::number_of_matches(7);
    assert_eq!(6, result);
}

#[test]
fn test2() {
    let result = Solution::number_of_matches(14);
    assert_eq!(13, result);
}
