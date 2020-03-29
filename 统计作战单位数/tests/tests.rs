extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::num_teams(vec![2, 5, 3, 4, 1]);
    assert_eq!(3, result);
}

#[test]
fn test2() {
    let result = Solution::num_teams(vec![2, 1, 3]);
    assert_eq!(0, result);
}

#[test]
fn test3() {
    let result = Solution::num_teams(vec![1, 2, 3, 4]);
    assert_eq!(4, result);
}
