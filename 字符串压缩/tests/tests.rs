extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::compress_string("aabcccccaaa".to_string());
    assert_eq!("a2b1c5a3".to_string(), result);
}

#[test]
fn test2() {
    let result = Solution::compress_string("abbccd".to_string());
    assert_eq!("abbccd".to_string(), result);
}
