extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]);
    assert_eq!(17, result);
}

#[test]
fn test2() {
    let result = Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]);
    assert_eq!(8, result);
}

#[test]
fn test3() {
    let result = Solution::maximum_unique_subarray(vec![10000]);
    assert_eq!(10000, result);
}
