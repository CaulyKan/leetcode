extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::longest_palindrome(String::from("babad"));
    assert_eq!("bab", result);
}

#[test]
fn test2() {
    let result = Solution::longest_palindrome(String::from("cbbd"));
    assert_eq!("bb", result);
}

#[test]
fn test3() {
    let result = Solution::longest_palindrome(String::from("a"));
    assert_eq!("a", result);
}

#[test]
fn test4() {
    let result = Solution::longest_palindrome(String::from("aa"));
    assert_eq!("aa", result);
}

#[test]
fn test5() {
    let result = Solution::longest_palindrome(String::from("aaa"));
    assert_eq!("aaa", result);
}

#[test]
fn test6() {
    let result = Solution::longest_palindrome(String::from("aaaa"));
    assert_eq!("aaaa", result);
}

#[test]
fn test7() {
    let result = Solution::longest_palindrome(String::from("aaaaa"));
    assert_eq!("aaaaa", result);
}

#[test]
fn test8() {
    let result = Solution::longest_palindrome(String::from("bananas"));
    assert_eq!("anana", result);
}

#[test]
fn test9() {
    let result = Solution::longest_palindrome(String::from("ababababababa"));
    assert_eq!("ababababababa", result);
}

#[test]
fn test10() {
    let result = Solution::longest_palindrome(String::from("aabbccbbaaaabbccbbaa"));
    assert_eq!("aabbccbbaaaabbccbbaa", result);
}

#[test]
fn test11() {
    let result = Solution::longest_palindrome(String::from("11111111111111111"));
    assert_eq!("11111111111111111", result);
}
