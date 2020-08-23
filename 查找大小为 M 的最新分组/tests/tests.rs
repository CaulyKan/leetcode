extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::find_latest_step(vec![3, 5, 1, 2, 4], 1);
    assert_eq!(4, result);
}

#[test]
fn test2() {
    let result = Solution::find_latest_step(vec![3, 1, 5, 4, 2], 2);
    assert_eq!(-1, result);
}

#[test]
fn test3() {
    let result = Solution::find_latest_step(vec![1], 1);
    assert_eq!(1, result);
}

#[test]
fn test4() {
    let result = Solution::find_latest_step(vec![2, 1], 2);
    assert_eq!(2, result);
}
