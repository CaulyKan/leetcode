extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::surface_area(vec![vec![2]]);
    assert_eq!(10, result);
}

#[test]
fn test2() {
    let result = Solution::surface_area(vec![vec![1, 2], vec![3, 4]]);
    assert_eq!(34, result);
}

#[test]
fn test3() {
    let result = Solution::surface_area(vec![vec![1, 0], vec![0, 2]]);
    assert_eq!(16, result);
}

#[test]
fn test4() {
    let result = Solution::surface_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]);
    assert_eq!(32, result);
}

#[test]
fn test5() {
    let result = Solution::surface_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]);
    assert_eq!(46, result);
}
