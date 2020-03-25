/*
 * @lc app=leetcode.cn id=114 lang=rust
 *
 * [114] 二叉树展开为链表
 */
pub struct Solution;
// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = root {
            TreeNode::flatten(Some(Rc::clone(n)));
        }
    }
}

impl TreeNode {
    pub fn flatten(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root != None {
            if let Some(left) = root.get_left() {
                let left_head = Some(Rc::clone(&left));
                let left_last = TreeNode::flatten(Some(left));
                if let Some(right) = root.get_right() {
                    let right_head = Some(Rc::clone(&right));
                    let right_last = TreeNode::flatten(Some(right));
                    left_last.set_sub_nodes(&None, &right_head);
                    root.set_sub_nodes(&None, &left_head);
                    right_last
                } else {
                    root.set_sub_nodes(&None, &left_head);
                    left_last
                }
            } else if let Some(right) = root.get_right() {
                let right_head = Some(Rc::clone(&right));
                let right_last = TreeNode::flatten(Some(right));
                root.set_sub_nodes(&None, &right_head);
                right_last
            } else {
                root
            }
        } else {
            None
        }
    }

    pub fn from_string(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        let v: Vec<&str> = s.split(',').collect();
        if v.len() == 0 {
            None
        } else {
            let root = Rc::new(RefCell::new(TreeNode::new(v[0].parse().unwrap())));
            let mut last_row = vec![Rc::clone(&root)];
            let mut iter = v.iter().skip(1);
            'outer: loop {
                let mut new_row = Vec::new();
                for parent in last_row.iter_mut() {
                    let left = match iter.next() {
                        Some(&"null") => None,
                        Some(num) => {
                            Some(Rc::from(RefCell::from(TreeNode::new(num.parse().unwrap()))))
                        }
                        _ => break 'outer,
                    };
                    let right = match iter.next() {
                        Some(&"null") => None,
                        Some(num) => {
                            Some(Rc::from(RefCell::from(TreeNode::new(num.parse().unwrap()))))
                        }
                        _ => break 'outer,
                    };
                    Some(Rc::clone(parent)).set_sub_nodes(&left, &right);
                    if left != None {
                        new_row.push(left.unwrap());
                    }
                    if right != None {
                        new_row.push(right.unwrap());
                    }
                }
                last_row = new_row;
                if last_row.len() == 0 {
                    break;
                }
            }

            Some(root)
        }
    }

    pub fn walk(root: &Option<Rc<RefCell<TreeNode>>>, func: fn(Option<Rc<RefCell<TreeNode>>>)) {
        if let Some(node) = root {
            func(Some(Rc::clone(&node)));
            let n = node.borrow();
            if let Some(left) = &n.left {
                TreeNode::walk(&Some(Rc::clone(left)), func);
            }
            if let Some(right) = &n.right {
                TreeNode::walk(&Some(Rc::clone(right)), func);
            }
        }
    }
    pub fn walk_return<T>(
        root: &Option<Rc<RefCell<TreeNode>>>,
        val_func: fn(Option<Rc<RefCell<TreeNode>>>) -> T,
        aggr_func: fn(T, Option<T>, Option<T>) -> Option<T>,
    ) -> Option<T> {
        if let Some(node) = root {
            let n = node.borrow();
            let mut lval = None;
            let mut rval = None;
            if let Some(left) = &n.left {
                lval = TreeNode::walk_return(&Some(Rc::clone(left)), val_func, aggr_func);
            }
            if let Some(right) = &n.right {
                rval = TreeNode::walk_return(&Some(Rc::clone(right)), val_func, aggr_func);
            }
            let val = val_func(Some(Rc::clone(&node)));
            aggr_func(val, lval, rval)
        } else {
            None
        }
    }
}

pub trait RefTreeNode {
    fn get_val(&self) -> Option<i32>;
    fn get_left(&self) -> Option<Rc<RefCell<TreeNode>>>;
    fn get_right(&self) -> Option<Rc<RefCell<TreeNode>>>;
    fn set_val(&mut self, v: i32);
    fn set_sub_nodes(
        &self,
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    );
}

impl RefTreeNode for Option<Rc<RefCell<TreeNode>>> {
    fn get_val(&self) -> Option<i32> {
        if let Some(n) = self {
            let n = n.borrow();
            Some(n.val)
        } else {
            None
        }
    }
    fn get_left(&self) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = self {
            let n = n.borrow();
            if let Some(l) = &n.left {
                return Some(Rc::clone(&l));
            }
        }
        None
    }
    fn get_right(&self) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = self {
            let n = n.borrow();
            if let Some(l) = &n.right {
                return Some(Rc::clone(&l));
            }
        }
        None
    }
    fn set_val(&mut self, v: i32) {
        if let Some(n) = self {
            let mut n = n.borrow_mut();
            n.val = v;
        }
    }
    fn set_sub_nodes(
        &self,
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(n) = self {
            let mut n = n.borrow_mut();
            n.left = match left {
                None => None,
                Some(n) => Some(Rc::clone(n)),
            };
            n.right = match right {
                None => None,
                Some(n) => Some(Rc::clone(n)),
            };
        }
    }
}

// @lc code=end

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
