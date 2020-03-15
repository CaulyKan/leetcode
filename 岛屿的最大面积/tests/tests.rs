extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let grid = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ];
    let result = Solution::max_area_of_island(grid);
    assert_eq!(6, result);
}

#[test]
fn test2() {
    let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];
    let result = Solution::max_area_of_island(grid);
    assert_eq!(0, result);
}

#[test]
fn test3() {
    let grid = vec![vec![1]];
    let result = Solution::max_area_of_island(grid);
    assert_eq!(1, result);
}
