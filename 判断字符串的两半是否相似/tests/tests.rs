extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::halves_are_alike("book".to_string());
    assert_eq!(true, result);
}

#[test]
fn test2() {
    let result = Solution::halves_are_alike("textbook".to_string());
    assert_eq!(false, result);
}
