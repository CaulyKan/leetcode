extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::count_pairs(vec![1, 3, 5, 7, 9]);
    assert_eq!(4, result);
}

#[test]
fn test2() {
    let result = Solution::count_pairs(vec![1, 1, 1, 3, 3, 3, 7]);
    assert_eq!(15, result);
}

#[test]
fn test3() {
    let result = Solution::count_pairs(vec![0, 0]);
    assert_eq!(0, result);
}

#[test]
fn test4() {
    let result = Solution::count_pairs(vec![0]);
    assert_eq!(0, result);
}

#[test]
fn test5() {
    let result = Solution::count_pairs(vec![0, 1]);
    assert_eq!(1, result);
}
