extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let mut result = Solution::split_array(vec![2, 19, 7, 13, 17, 3]);
    assert_eq!(2, result);
}

#[test]
fn test2() {
    let mut result = Solution::split_array(vec![2, 3, 3, 2, 3, 4]);
    assert_eq!(1, result);
}
