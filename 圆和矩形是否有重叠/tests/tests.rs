extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::check_overlap(1, 0, 0, 1, -1, 3, 1);
    assert_eq!(true, result);
}

#[test]
fn test2() {
    let result = Solution::check_overlap(1, 0, 0, -1, 0, 0, 1);
    assert_eq!(true, result);
}

#[test]
fn test3() {
    let result = Solution::check_overlap(1, 1, 1, -3, -3, 3, 3);
    assert_eq!(true, result);
}

#[test]
fn test4() {
    let result = Solution::check_overlap(1, 1, 1, 1, -3, 2, -1);
    assert_eq!(false, result);
}
