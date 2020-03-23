extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]);
    assert_eq!(4, result);
}

#[test]
fn test2() {
    let result = Solution::length_of_lis(vec![18]);
    assert_eq!(1, result);
}

#[test]
fn test3() {
    let result = Solution::length_of_lis(vec![]);
    assert_eq!(0, result);
}
