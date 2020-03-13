extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::majority_element(vec![3, 2, 3]);
    assert_eq!(3, result);
}

#[test]
fn test2() {
    let result = Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]);
    assert_eq!(2, result);
}

#[test]
fn test4() {
    let result = Solution::majority_element(vec![6, 5, 5]);
    assert_eq!(5, result);
}

#[test]
fn test3() {
    let result = Solution::majority_element(vec![1]);
    assert_eq!(1, result);
}
