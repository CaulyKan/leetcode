extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::find_continuous_sequence(9);
    assert_eq!(vec![2, 3, 4], result[0]);
    assert_eq!(vec![4, 5], result[1]);
}

#[test]
fn test2() {
    let result = Solution::find_continuous_sequence(15);
    assert_eq!(vec![7, 8], result[2]);
    assert_eq!(vec![4, 5, 6], result[1]);
    assert_eq!(vec![1, 2, 3, 4, 5], result[0]);
}

#[test]
fn test3() {
    let result = Solution::find_continuous_sequence(1);
    assert_eq!(0, result.len());
}
