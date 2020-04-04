extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]);
    assert_eq!(14, result);
}

#[test]
fn test2() {
    let result = Solution::max_satisfaction(vec![4, 3, 2]);
    assert_eq!(20, result);
}

#[test]
fn test3() {
    let result = Solution::max_satisfaction(vec![-1, -4, -5]);
    assert_eq!(0, result);
}

#[test]
fn test4() {
    let result = Solution::max_satisfaction(vec![-2, 5, -1, 0, 3, -3]);
    assert_eq!(35, result);
}
