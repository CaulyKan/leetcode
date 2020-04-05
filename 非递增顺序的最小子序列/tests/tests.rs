extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::min_subsequence(vec![4, 3, 10, 9, 8]);
    assert_eq!(vec![10, 9], result);
}

#[test]
fn test2() {
    let result = Solution::min_subsequence(vec![4, 4, 7, 6, 7]);
    assert_eq!(vec![7, 7, 6], result);
}

#[test]
fn test3() {
    let result = Solution::min_subsequence(vec![6]);
    assert_eq!(vec![6], result);
}
