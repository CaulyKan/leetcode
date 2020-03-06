extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::length_of_longest_substring("abcabcbb".to_string());
    assert_eq!(3, result);
}

#[test]
fn test2() {
    let result = Solution::length_of_longest_substring("bbbbb".to_string());
    assert_eq!(1, result);
}

#[test]
fn test3() {
    let result = Solution::length_of_longest_substring("pwwkew".to_string());
    assert_eq!(3, result);
}

#[test]
fn test4() {
    let result = Solution::length_of_longest_substring("".to_string());
    assert_eq!(0, result);
}

#[test]
fn test5() {
    let result = Solution::length_of_longest_substring(" ".to_string());
    assert_eq!(1, result);
}

#[test]
fn test6() {
    let result = Solution::length_of_longest_substring("ab".to_string());
    assert_eq!(2, result);
}

#[test]
fn test7() {
    let result = Solution::length_of_longest_substring("aa".to_string());
    assert_eq!(1, result);
}
