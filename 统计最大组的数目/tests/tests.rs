extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::count_largest_group(13);
    assert_eq!(4, result);
}

#[test]
fn test2() {
    let result = Solution::count_largest_group(2);
    assert_eq!(2, result);
}

#[test]
fn test3() {
    let result = Solution::count_largest_group(15);
    assert_eq!(6, result);
}

#[test]
fn test4() {
    let result = Solution::count_largest_group(24);
    assert_eq!(5, result);
}
