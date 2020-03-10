/*
 * @lc app=leetcode.cn id=543 lang=rust
 *
 * [543] 二叉树的直径
 */

// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;

        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();

                let lmax = helper(&node.left, max);
                let rmax = helper(&node.right, max);

                let current = lmax + rmax;
                *max = std::cmp::max(*max, current);
                std::cmp::max(lmax, rmax) + 1
            } else {
                0
            }
        }
        helper(&root, &mut max);
        max
    }
}
// @lc code=end

pub struct Solution;

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
