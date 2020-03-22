extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::longest_prefix("level".to_string());
    assert_eq!("l", result);
}

#[test]
fn test2() {
    let result = Solution::longest_prefix("ababab".to_string());
    assert_eq!("abab", result);
}

#[test]
fn test3() {
    let result = Solution::longest_prefix("leetcodeleet".to_string());
    assert_eq!("leet", result);
}
