extern crate leetcode;
use leetcode::MajorityChecker;

#[test]
fn test1() {
    let mut obj = MajorityChecker::new(vec![1, 1, 2, 2, 1, 1]);
    let result = obj.query(0, 5, 4);
    assert_eq!(1, result);
}

#[test]
fn test2() {
    let mut obj = MajorityChecker::new(vec![1, 1, 2, 2, 1, 1]);
    let result = obj.query(0, 3, 3);
    assert_eq!(-1, result);
}

#[test]
fn test3() {
    let mut obj = MajorityChecker::new(vec![1, 1, 2, 2, 1, 1]);
    let result = obj.query(2, 3, 2);
    assert_eq!(2, result);
}
