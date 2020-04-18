extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::min_count(vec![4, 2, 1]);
    assert_eq!(4, result);
}

#[test]
fn test2() {
    let result = Solution::min_count(vec![2, 3, 10]);
    assert_eq!(8, result);
}
