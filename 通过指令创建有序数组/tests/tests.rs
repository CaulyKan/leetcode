extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::create_sorted_array(vec![1, 5, 6, 2]);
    assert_eq!(1, result);
}

#[test]
fn test2() {
    let result = Solution::create_sorted_array(vec![1, 2, 3, 6, 5, 4]);
    assert_eq!(3, result);
}

#[test]
fn test3() {
    let result = Solution::create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2]);
    assert_eq!(4, result);
}
