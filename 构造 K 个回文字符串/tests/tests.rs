extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::can_construct("annabelle".to_string(), 2);
    assert_eq!(true, result);
}

#[test]
fn test2() {
    let result = Solution::can_construct("leetcode".to_string(), 3);
    assert_eq!(false, result);
}

#[test]
fn test3() {
    let result = Solution::can_construct("true".to_string(), 4);
    assert_eq!(true, result);
}

#[test]
fn test4() {
    let result = Solution::can_construct("yzyzyzyzyzyzyzy".to_string(), 2);
    assert_eq!(true, result);
}

#[test]
fn test5() {
    let result = Solution::can_construct("cr".to_string(), 7);
    assert_eq!(false, result);
}
