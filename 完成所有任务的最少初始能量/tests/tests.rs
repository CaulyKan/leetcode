extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::minimum_effort(vec![vec![1, 1], vec![1, 3]]);
    assert_eq!(3, result);
}
