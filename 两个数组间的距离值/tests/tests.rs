extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2);
    assert_eq!(2, result);
}

#[test]
fn test2() {
    let result =
        Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3);
    assert_eq!(2, result);
}

#[test]
fn test3() {
    let result = Solution::find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6);
    assert_eq!(1, result);
}
