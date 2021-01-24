extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::largest_submatrix(vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]]);
    assert_eq!(4, result);
}

#[test]
fn test2() {
    let result = Solution::largest_submatrix(vec![vec![1, 0, 1, 0, 1]]);
    assert_eq!(3, result);
}

#[test]
fn test3() {
    let result = Solution::largest_submatrix(vec![vec![1, 1, 0], vec![1, 0, 1]]);
    assert_eq!(2, result);
}

#[test]
fn test4() {
    let result = Solution::largest_submatrix(vec![vec![0, 0], vec![0, 0]]);
    assert_eq!(0, result);
}
