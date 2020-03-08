extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
    assert_eq!(2.0, result);
}

#[test]
fn test2() {
    let result = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
    assert_eq!(2.5, result);
}

#[test]
fn test3() {
    let result = Solution::find_median_sorted_arrays(vec![1, 2], vec![]);
    assert_eq!(1.5, result);
}
