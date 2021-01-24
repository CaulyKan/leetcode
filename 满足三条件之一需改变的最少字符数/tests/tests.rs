extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::min_characters("aba".to_string(), "caa".to_string());
    assert_eq!(2, result);
}

#[test]
fn test2() {
    let result = Solution::min_characters("dabadd".to_string(), "cda".to_string());
    assert_eq!(3, result);
}

#[test]
fn test3() {
    let result = Solution::min_characters("zzzzzzzzzzzzzzzzz".to_string(), "z".to_string());
    assert_eq!(0, result);
}

#[test]
fn test4() {
    let result =
        Solution::min_characters("abcdefghijklmnopqrstuvwxyz".to_string(), "z".to_string());
    assert_eq!(1, result);
}

#[test]
fn test5() {
    let result = Solution::min_characters("abc".to_string(), "abc".to_string());
    assert_eq!(3, result);
}

#[test]
fn test6() {
    let result = Solution::min_characters("a".to_string(), "aaaabcdefghijklmnzzzzz".to_string());
    assert_eq!(4, result);
}
