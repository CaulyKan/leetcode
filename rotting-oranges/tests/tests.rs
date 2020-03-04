extern crate rotting_oranges;

#[test]
fn test1() {
    let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    let result = rotting_oranges::Solution::oranges_rotting(grid);
    assert_eq!(4, result);
}

#[test]
fn test2() {
    let grid = vec![vec![2, 1, 1], vec![0, 1, 0], vec![1, 0, 1]];
    let result = rotting_oranges::Solution::oranges_rotting(grid);
    assert_eq!(-1, result);
}

#[test]
fn test3() {
    let grid = vec![vec![0, 2]];
    let result = rotting_oranges::Solution::oranges_rotting(grid);
    assert_eq!(0, result);
}
