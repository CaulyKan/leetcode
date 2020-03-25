extern crate leetcode_utils;
use leetcode_utils::linked_list::*;

#[test]
fn test1() {
    let result = TreeNode::from_string("1").unwrap();
    assert_eq!(1, result.borrow().val);
}

#[test]
fn test2() {
    let result = TreeNode::from_string("1,2,3");
    TreeNode::walk(&result, |mut x| x.set_val(x.get_val().unwrap() + 1));
    assert_eq!(Some(2), result.get_val());
    assert_eq!(Some(3), result.get_left().get_val());
    assert_eq!(Some(4), result.get_right().get_val());
}

#[test]
fn test3() {
    let result = TreeNode::from_string("1,null,3,4,5");
    assert_eq!(Some(1), result.get_val());
    assert_eq!(None, result.get_left());
    assert_eq!(Some(3), result.get_right().get_val());
    assert_eq!(Some(4), result.get_right().get_left().get_val());
    assert_eq!(Some(5), result.get_right().get_right().get_val());
}

#[test]
fn test4() {
    let result = TreeNode::from_string("1,null,null");
    assert_eq!(Some(1), result.get_val());
    assert_eq!(None, result.get_left());
    assert_eq!(None, result.get_right());
}

#[test]
fn test5() {
    let result = TreeNode::from_string("1,2,3,4,5");
    let sum = TreeNode::walk_return(
        &result,
        |n| n.get_val().unwrap_or(0),
        |v, l, r| Some(v + l.unwrap_or(0) + r.unwrap_or(0)),
    );
    assert_eq!(Some(1 + 2 + 3 + 4 + 5), sum);
}
