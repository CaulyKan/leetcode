extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::num_of_minutes(1, 0, vec![-1], vec![0]);
    assert_eq!(0, result);
}

#[test]
fn test2() {
    let result = Solution::num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0]);
    assert_eq!(1, result);
}
#[test]
fn test3() {
    let result =
        Solution::num_of_minutes(7, 6, vec![1, 2, 3, 4, 5, 6, -1], vec![0, 6, 5, 4, 3, 2, 1]);
    assert_eq!(21, result);
}

#[test]
fn test4() {
    let result = Solution::num_of_minutes(4, 2, vec![3, 3, -1, 2], vec![0, 0, 162, 914]);
    assert_eq!(1076, result);
}
