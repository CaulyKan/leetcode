extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::min_number_of_frogs("croakcroak".to_string());
    assert_eq!(1, result);
}

#[test]
fn test2() {
    let result = Solution::min_number_of_frogs("crcoakroak".to_string());
    assert_eq!(2, result);
}

#[test]
fn test3() {
    let result = Solution::min_number_of_frogs("croakcrook".to_string());
    assert_eq!(-1, result);
}

#[test]
fn test4() {
    let result = Solution::min_number_of_frogs("croakcroa".to_string());
    assert_eq!(-1, result);
}
