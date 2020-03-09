extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
    assert_eq!(5, result);
}

#[test]
fn test2() {
    let result = Solution::max_profit(vec![7, 6, 4, 3, 1]);
    assert_eq!(0, result);
}

#[test]
fn test3() {
    let result = Solution::max_profit(vec![1]);
    assert_eq!(0, result);
}

#[test]
fn test4() {
    let result = Solution::max_profit(Vec::<i32>::new());
    assert_eq!(0, result);
}

#[test]
fn test5() {
    let result = Solution::max_profit(vec![1, 2]);
    assert_eq!(1, result);
}
