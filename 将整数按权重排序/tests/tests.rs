extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::get_kth(12, 15, 2);
    assert_eq!(13, result);
}

#[test]
fn test2() {
    let result = Solution::get_kth(1, 1, 1);
    assert_eq!(1, result);
}

#[test]
fn test3() {
    let result = Solution::get_kth(7, 11, 4);
    assert_eq!(7, result);
}

#[test]
fn test4() {
    let result = Solution::get_kth(10, 20, 5);
    assert_eq!(13, result);
}

#[test]
fn test5() {
    let result = Solution::get_kth(1, 1000, 777);
    assert_eq!(570, result);
}
