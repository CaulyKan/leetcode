extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::store_water(vec![9, 0, 1], vec![0, 2, 2]);
    assert_eq!(3, result);
}

#[test]
fn test2() {
    let result = Solution::store_water(vec![1, 3], vec![6, 8]);
    assert_eq!(4, result);
}

#[test]
fn test3() {
    let result = Solution::store_water(vec![0], vec![0]);
    assert_eq!(0, result);
}
