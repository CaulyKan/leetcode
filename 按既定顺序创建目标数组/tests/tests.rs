extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]);
    assert_eq!(vec![0, 4, 1, 3, 2], result);
}

#[test]
fn test2() {
    let result = Solution::create_target_array(vec![1], vec![0]);
    assert_eq!(vec![1], result);
}
