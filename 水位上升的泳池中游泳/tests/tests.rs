extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::swim_in_water(vec![vec![0, 2], vec![1, 3]]);
    assert_eq!(3, result);
}

#[test]
fn test2() {
    let result = Solution::swim_in_water(vec![
        vec![0, 1, 2, 3, 4],
        vec![24, 23, 22, 21, 5],
        vec![12, 13, 14, 15, 16],
        vec![11, 17, 18, 19, 20],
        vec![10, 9, 8, 7, 6],
    ]);
    assert_eq!(16, result);
}
