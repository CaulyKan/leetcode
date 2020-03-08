extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::coin_change(vec![1, 2, 5], 11);
    assert_eq!(3, result);
}

#[test]
fn test2() {
    let result = Solution::coin_change(vec![2], 3);
    assert_eq!(-1, result);
}

#[test]
fn test3() {
    let result = Solution::coin_change(vec![2], 1);
    assert_eq!(-1, result);
}

#[test]
fn test4() {
    let result = Solution::coin_change(vec![1], 1);
    assert_eq!(1, result);
}

#[test]
fn test5() {
    let result = Solution::coin_change(vec![1, 2, 5], 10);
    assert_eq!(2, result);
}
