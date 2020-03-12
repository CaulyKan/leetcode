extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string());
    assert_eq!("ABC", result);
}

#[test]
fn test2() {
    let result = Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string());
    assert_eq!("AB", result);
}

#[test]
fn test3() {
    let result = Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string());
    assert_eq!("", result);
}

#[test]
fn test4() {
    let result = Solution::gcd_of_strings(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        "aaaaaaaaaaaaaa".to_string(),
    );
    assert_eq!("aaaaaaaaaaaaaa", result);
}
