extern crate leetcode;
use leetcode::*;

#[test]
fn test1() {
    let result = Solution::max_path_sum(TreeNode::from_string("1,2,3"));
    assert_eq!(6, result);
}

#[test]
fn test2() {
    let result = Solution::max_path_sum(TreeNode::from_string("-10,9,20,null,null,15,7"));
    assert_eq!(42, result);
}
