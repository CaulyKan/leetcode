extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let matrix = vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]];
    let result = Solution::lucky_numbers(matrix);
    assert_eq!(vec![15], result);
}

#[test]
fn test2() {
    let matrix = vec![vec![1, 10, 4, 2], vec![9, 3, 8, 7], vec![15, 16, 17, 12]];
    let result = Solution::lucky_numbers(matrix);
    assert_eq!(vec![12], result);
}
