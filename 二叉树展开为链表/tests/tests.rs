extern crate leetcode;
use leetcode::*;

#[test]
fn test1() {
    let mut tree = TreeNode::from_string("1,2,5,3,4,null,6");
    Solution::flatten(&mut tree);
    assert_eq!(Some(1), tree.get_val());
    assert_eq!(None, tree.get_left().get_val());
    assert_eq!(Some(2), tree.get_right().get_val());
    assert_eq!(None, tree.get_right().get_left().get_val());
    assert_eq!(Some(3), tree.get_right().get_right().get_val());
}
