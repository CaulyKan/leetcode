extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2);
    assert_eq!(7, result);
}

#[test]
fn test2() {
    let result = Solution::max_result(vec![10, -5, -2, 4, 0, 3], 3);
    assert_eq!(17, result);
}

#[test]
fn test3() {
    let result = Solution::max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2);
    assert_eq!(0, result);
}

#[test]
fn test4() {
    let result = Solution::max_result(vec![1], 1);
    assert_eq!(1, result);
}
