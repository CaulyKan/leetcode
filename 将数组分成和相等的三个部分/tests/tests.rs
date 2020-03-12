extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]);
    assert_eq!(true, result);
}

#[test]
fn test2() {
    let result = Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1]);
    assert_eq!(false, result);
}

#[test]
fn test3() {
    let result = Solution::can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4]);
    assert_eq!(true, result);
}

#[test]
fn test4() {
    let result = Solution::can_three_parts_equal_sum(vec![1, -1, 1, -1]);
    assert_eq!(false, result);
}
