extern crate leetcode;
use leetcode::*;

#[test]
fn test1() {
    let mut l1 = Box::from(ListNode::new(2));
    l1.next = Some(Box::from(ListNode::new(4)));
    l1.next.as_mut().unwrap().next = Some(Box::from(ListNode::new(3)));
    let mut l2 = Box::from(ListNode::new(5));
    l2.next = Some(Box::from(ListNode::new(6)));
    l2.next.as_mut().unwrap().next = Some(Box::from(ListNode::new(4)));

    let result = Solution::add_two_numbers(Some(l1), Some(l2));
    assert_eq!(7, result.as_ref().unwrap().val);
    assert_eq!(0, result.as_ref().unwrap().next.as_ref().unwrap().val);
    assert_eq!(
        8,
        result
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .val
    );
}

#[test]
fn test2() {
    let mut l1 = Box::from(ListNode::new(2));
    l1.next = Some(Box::from(ListNode::new(4)));
    let mut l2 = Box::from(ListNode::new(5));
    l2.next = Some(Box::from(ListNode::new(6)));

    let result = Solution::add_two_numbers(Some(l1), Some(l2));
    assert_eq!(7, result.as_ref().unwrap().val);
    assert_eq!(0, result.as_ref().unwrap().next.as_ref().unwrap().val);
    assert_eq!(
        1,
        result
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .val
    );
}
