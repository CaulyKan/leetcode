extern crate leetcode;
use leetcode::MaxQueue;

#[test]
fn test1() {
    let mut result = MaxQueue::new();
    result.push_back(1);
    result.push_back(2);
    assert_eq!(2, result.max_value());
    assert_eq!(1, result.pop_front());
    assert_eq!(2, result.max_value());
}

#[test]
fn test2() {
    let mut result = MaxQueue::new();
    assert_eq!(-1, result.pop_front());
    assert_eq!(-1, result.max_value());
}

#[test]
fn test3() {
    let mut result = MaxQueue::new();
    result.push_back(1);
    result.push_back(2);
    result.push_back(0);
    assert_eq!(2, result.max_value());
    assert_eq!(1, result.pop_front());
    assert_eq!(2, result.max_value());
    assert_eq!(2, result.pop_front());
    assert_eq!(0, result.max_value());
}
