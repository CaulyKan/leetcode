extern crate leetcode;
use leetcode::*;

#[test]
fn test1() {
    let tree = TreeNode::from_string("1,2,3");
    let result = Solution::inorder_traversal(tree);
    assert_eq!(vec![2, 1, 3], result);
}

#[test]
fn test2() {
    let tree = TreeNode::from_string("1,null,2,3");
    let result = Solution::inorder_traversal(tree);
    assert_eq!(vec![1, 3, 2], result);
}

#[test]
fn test3() {
    let tree = None;
    let result = Solution::inorder_traversal(tree);
    assert_eq!(Vec::<i32>::new(), result);
}

#[test]
fn test4() {
    let tree = TreeNode::from_string("1");
    let result = Solution::inorder_traversal(tree);
    assert_eq!(vec![1], result);
}
