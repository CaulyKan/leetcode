/*
 * @lc app=leetcode.cn id=124 lang=rust
 *
 * [124] 二叉树中的最大路径和
 */

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
use std::cmp;
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = i32::min_value();
        root.aggregate_t(
            &mut max,
            |_, node| node.get_val().unwrap_or(0),
            |max, val, lval, rval| {
                *max = cmp::max(*max, val);
                *max = cmp::max(*max, val + lval.unwrap_or(0));
                *max = cmp::max(*max, val + rval.unwrap_or(0));
                *max = cmp::max(*max, val + lval.unwrap_or(0) + rval.unwrap_or(0));

                Some(cmp::max(
                    val,
                    cmp::max(val + lval.unwrap_or(0), val + rval.unwrap_or(0)),
                ))
            },
        );

        max
    }
}

// #region Tree ext

#[derive(PartialEq, Copy, Clone)]
pub enum TraversalType {
    Inorder,
    Preorder,
    Postorder,
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
    fn walk(&self, t: TraversalType, func: fn(Option<Rc<RefCell<TreeNode>>>));
    fn walk_t<T>(
        &self,
        t: TraversalType,
        result: &mut T,
        func: fn(&mut T, Option<Rc<RefCell<TreeNode>>>),
    );
    fn aggregate<T>(
        &self,
        val_func: fn(Option<Rc<RefCell<TreeNode>>>) -> T,
        aggr_func: fn(T, Option<T>, Option<T>) -> Option<T>,
    ) -> Option<T>;
    fn aggregate_t<R, T>(
        &self,
        result: &mut R,
        val_func: fn(&mut R, Option<Rc<RefCell<TreeNode>>>) -> T,
        aggr_func: fn(&mut R, T, Option<T>, Option<T>) -> Option<T>,
    ) -> Option<T>;
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
    fn walk(&self, t: TraversalType, func: fn(Option<Rc<RefCell<TreeNode>>>)) {
        if let Some(node) = self {
            if t == TraversalType::Preorder {
                func(Some(Rc::clone(&node)));
            }
            {
                let n = node.borrow();
                if let Some(left) = &n.left {
                    Some(Rc::clone(left)).walk(t, func);
                }
            }
            if t == TraversalType::Inorder {
                func(Some(Rc::clone(&node)));
            }
            {
                let n = node.borrow();
                if let Some(right) = &n.right {
                    Some(Rc::clone(right)).walk(t, func);
                }
            }
            if t == TraversalType::Postorder {
                func(Some(Rc::clone(&node)));
            }
        }
    }
    fn walk_t<T>(
        &self,
        t: TraversalType,
        result: &mut T,
        func: fn(&mut T, Option<Rc<RefCell<TreeNode>>>),
    ) {
        if let Some(node) = self {
            if t == TraversalType::Preorder {
                func(result, Some(Rc::clone(&node)));
            }
            {
                let n = node.borrow();
                if let Some(left) = &n.left {
                    Some(Rc::clone(left)).walk_t(t, result, func);
                }
            }
            if t == TraversalType::Inorder {
                func(result, Some(Rc::clone(&node)));
            }
            {
                let n = node.borrow();
                if let Some(right) = &n.right {
                    Some(Rc::clone(right)).walk_t(t, result, func);
                }
            }
            if t == TraversalType::Postorder {
                func(result, Some(Rc::clone(&node)));
            }
        }
    }
    fn aggregate<T>(
        &self,
        val_func: fn(Option<Rc<RefCell<TreeNode>>>) -> T,
        aggr_func: fn(T, Option<T>, Option<T>) -> Option<T>,
    ) -> Option<T> {
        if let Some(node) = self {
            let n = node.borrow();
            let mut lval = None;
            let mut rval = None;
            if let Some(left) = &n.left {
                lval = Some(Rc::clone(left)).aggregate(val_func, aggr_func);
            }
            if let Some(right) = &n.right {
                rval = Some(Rc::clone(right)).aggregate(val_func, aggr_func);
            }
            let val = val_func(Some(Rc::clone(&node)));
            aggr_func(val, lval, rval)
        } else {
            None
        }
    }
    fn aggregate_t<R, T>(
        &self,
        result: &mut R,
        val_func: fn(&mut R, Option<Rc<RefCell<TreeNode>>>) -> T,
        aggr_func: fn(&mut R, T, Option<T>, Option<T>) -> Option<T>,
    ) -> Option<T> {
        if let Some(node) = self {
            let n = node.borrow();
            let mut lval = None;
            let mut rval = None;
            if let Some(left) = &n.left {
                lval = Some(Rc::clone(left)).aggregate_t(result, val_func, aggr_func);
            }
            if let Some(right) = &n.right {
                rval = Some(Rc::clone(right)).aggregate_t(result, val_func, aggr_func);
            }
            let val = val_func(result, Some(Rc::clone(&node)));
            aggr_func(result, val, lval, rval)
        } else {
            None
        }
    }
}

impl TreeNode {
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
                        _ => None,
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
}
// #endregion

// @lc code=end

pub struct Solution;

// #region LeetCode Tree
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
// endregion
