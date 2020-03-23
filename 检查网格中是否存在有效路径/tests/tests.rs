extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::has_valid_path(vec![vec![2, 4, 3], vec![6, 5, 2]]);
    assert_eq!(true, result);
}

#[test]
fn test2() {
    let result = Solution::has_valid_path(vec![vec![1, 1, 2]]);
    assert_eq!(false, result);
}

#[test]
fn test3() {
    let result = Solution::has_valid_path(vec![
        vec![6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
        vec![4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5],
        vec![6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
        vec![4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5],
        vec![6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
        vec![4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5],
        vec![6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
        vec![4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5],
        vec![6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
        vec![4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5],
        vec![6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
        vec![4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5],
        vec![6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
        vec![4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5],
        vec![6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
    ]);
    assert_eq!(true, result);
}
