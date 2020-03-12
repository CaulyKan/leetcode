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
    let mut n1 = Some(Rc::from(RefCell::from(TreeNode::new(1))));
    let n2 = Some(Rc::from(RefCell::from(TreeNode::new(1))));
    let mut n3 = Some(Rc::from(RefCell::from(TreeNode::new(1))));
    let mut n4 = Some(Rc::from(RefCell::from(TreeNode::new(1))));
    let n5 = Some(Rc::from(RefCell::from(TreeNode::new(1))));
    let n6 = Some(Rc::from(RefCell::from(TreeNode::new(1))));
    set_sub_nodes(&mut n4, None, n6);
    set_sub_nodes(&mut n3, n4, n5);
    set_sub_nodes(&mut n1, None, n3);
    set_sub_nodes(&mut root, n1, n2);

    let result = Solution::longest_zig_zag(root);
    assert_eq!(4, result);
}
