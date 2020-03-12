// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;

        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> (i32, i32) {
            if let Some(node) = root {
                let node = node.borrow();

                let (_, lr) = helper(&node.left, max);
                let (rl, _) = helper(&node.right, max);

                let current = std::cmp::max(lr, rl);
                *max = std::cmp::max(*max, current);
                (lr + 1, rl + 1)
            } else {
                (0, 0)
            }
        }
        helper(&root, &mut max);
        max
    }
}
