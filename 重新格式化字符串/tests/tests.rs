extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::reformat("a0b1c2".to_string());
    assert_eq!("a0b1c2".to_string(), result);
}

#[test]
fn test2() {
    let result = Solution::reformat("leetcode".to_string());
    assert_eq!("".to_string(), result);
}

#[test]
fn test3() {
    let result = Solution::reformat("covid2019".to_string());
    assert_eq!("c2o0v1i9d".to_string(), result);
}
