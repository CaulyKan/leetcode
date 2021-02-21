extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::minimum_effort_path(vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]]);
    assert_eq!(2, result);
}

#[test]
fn test2() {
    let result = Solution::minimum_effort_path(vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]]);
    assert_eq!(1, result);
}

#[test]
fn test3() {
    let result = Solution::minimum_effort_path(vec![vec![1, 10, 6, 7, 9, 10, 4, 9]]);
    assert_eq!(9, result);
}

#[test]
fn test4() {
    let result = Solution::minimum_effort_path(vec![
        vec![4, 3, 4, 10, 5, 5, 9, 2],
        vec![10, 8, 2, 10, 9, 7, 5, 6],
        vec![5, 8, 10, 10, 10, 7, 4, 2],
        vec![5, 1, 3, 1, 1, 3, 1, 9],
        vec![6, 4, 10, 6, 10, 9, 4, 6],
    ]);
    assert_eq!(5, result);
}
