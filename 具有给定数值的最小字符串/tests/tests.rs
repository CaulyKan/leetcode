extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::get_smallest_string(3, 27);
    assert_eq!("aay".to_string(), result);
}

#[test]
fn test2() {
    let result = Solution::get_smallest_string(5, 73);
    assert_eq!("aaszz".to_string(), result);
}
