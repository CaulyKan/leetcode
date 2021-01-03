extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4);
    assert_eq!(8, result);
}
