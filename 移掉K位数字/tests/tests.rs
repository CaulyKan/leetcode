extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::remove_kdigits(String::from("1432219"), 3);
    assert_eq!(String::from("1219"), result);
}

#[test]
fn test2() {
    let result = Solution::remove_kdigits(String::from("10200"), 1);
    assert_eq!(String::from("200"), result);
}

#[test]
fn test3() {
    let result = Solution::remove_kdigits(String::from("10"), 2);
    assert_eq!(String::from("0"), result);
}
