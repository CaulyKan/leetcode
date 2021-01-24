extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 1);
    assert_eq!(7, result);
}
