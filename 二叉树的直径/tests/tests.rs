extern crate leetcode;
use leetcode::Solution;
use leetcode::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn set_sub_nodes(
    root: &mut Option<Rc<RefCell<TreeNode>>>,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) {
    if let Some(n) = root {
        let mut n = n.borrow_mut();
        n.left = left;
        n.right = right;
    }
}

#[test]
fn test1() {
    let mut root = Some(Rc::from(RefCell::from(TreeNode::new(1))));
    let mut n2 = Some(Rc::from(RefCell::from(TreeNode::new(1))));
    let mut n3 = Some(Rc::from(RefCell::from(TreeNode::new(1))));
    let mut n4 = Some(Rc::from(RefCell::from(TreeNode::new(1))));
    let mut n5 = Some(Rc::from(RefCell::from(TreeNode::new(1))));
    set_sub_nodes(&mut n2, n4, n5);
    set_sub_nodes(&mut root, n2, n3);

    let result = Solution::diameter_of_binary_tree(root);
    assert_eq!(3, result);
}

#[test]
fn test2() {
    let mut root = Some(Rc::from(RefCell::from(TreeNode::new(1))));

    let result = Solution::diameter_of_binary_tree(root);
    assert_eq!(0, result);
}
