/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */

// @lc code=start

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;

        let mut first: Option<Box<ListNode>> = None;
        let mut current = &mut first;
        let mut carry = 0;
        loop {
            let val = match &l1 {
                Some(i) => i.val,
                _ => 0,
            } + match &l2 {
                Some(i) => i.val,
                _ => 0,
            } + carry;
            carry = val / 10;

            if current.is_none() {
                first = Some(Box::from(ListNode::new(val % 10)));
                current = &mut first;
            } else {
                current.as_mut().unwrap().next = Some(Box::from(ListNode::new(val % 10)));
                current = &mut current.as_mut().unwrap().next;
            }

            if l1.is_some() {
                l1 = l1.unwrap().next;
            }
            if l2.is_some() {
                l2 = l2.unwrap().next;
            }

            if l1.is_none() && l2.is_none() {
                break;
            }
        }
        if carry > 0 {
            current.as_mut().unwrap().next = Some(Box::from(ListNode::new(carry)));
        }

        return first;
    }
}

// @lc code=end

pub struct Solution;
//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
