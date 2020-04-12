extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::process_queries(vec![3, 1, 2, 1], 5);
    assert_eq!(vec![2, 1, 2, 1], result);
}

#[test]
fn test2() {
    let result = Solution::process_queries(vec![4, 1, 2, 2], 4);
    assert_eq!(vec![3, 1, 2, 0], result);
}

#[test]
fn test3() {
    let result = Solution::process_queries(vec![7, 5, 5, 8, 3], 8);
    assert_eq!(vec![6, 5, 0, 7, 5], result);
}
