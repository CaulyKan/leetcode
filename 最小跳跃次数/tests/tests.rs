extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::min_jump(vec![2, 5, 1, 1, 1, 1]);
    assert_eq!(3, result);
}

#[test]
fn test2() {
    let result = Solution::min_jump(vec![
        3, 7, 6, 1, 4, 3, 7, 8, 1, 2, 8, 5, 9, 8, 3, 2, 7, 5, 1, 1,
    ]);
    assert_eq!(6, result);
}

#[test]
fn test3() {
    let result = Solution::min_jump(vec![
        4, 6, 10, 8, 3, 5, 3, 5, 7, 8, 6, 10, 3, 7, 3, 10, 7, 10, 10, 9, 1, 4, 7, 4, 8, 6, 9, 8, 8,
        2, 7, 2, 4, 5, 4, 3, 3, 2, 2, 2, 3, 4, 4, 1, 1, 5, 6, 8, 1, 2,
    ]);
    assert_eq!(11, result);
}
