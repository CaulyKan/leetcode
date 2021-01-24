extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::tuple_same_product(vec![2, 3, 4, 6]);
    assert_eq!(8, result);
}

#[test]
fn test2() {
    let result = Solution::tuple_same_product(vec![1, 2, 4, 5, 10]);
    assert_eq!(16, result);
}

#[test]
fn test3() {
    let result = Solution::tuple_same_product(vec![2, 3, 4, 6, 8, 12]);
    assert_eq!(40, result);
}
