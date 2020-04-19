extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::get_happy_string(1, 3);
    assert_eq!("c", result);
}

#[test]
fn test2() {
    let result = Solution::get_happy_string(1, 4);
    assert_eq!("", result);
}

#[test]
fn test3() {
    let result = Solution::get_happy_string(3, 9);
    assert_eq!("cab", result);
}

#[test]
fn test4() {
    let result = Solution::get_happy_string(10, 100);
    assert_eq!("abacbabacb", result);
}
