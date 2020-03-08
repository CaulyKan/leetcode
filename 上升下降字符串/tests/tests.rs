extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::sort_string("aaaabbbbcccc".to_string());
    assert_eq!("abccbaabccba", result);
}

#[test]
fn test2() {
    let result = Solution::sort_string("rat".to_string());
    assert_eq!("art", result);
}
#[test]
fn test3() {
    let result = Solution::sort_string("leetcode".to_string());
    assert_eq!("cdelotee", result);
}

#[test]
fn test4() {
    let result = Solution::sort_string("rat".to_string());
    assert_eq!("art", result);
}
