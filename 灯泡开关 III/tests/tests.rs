extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::num_times_all_blue(vec![2, 1, 3, 5, 4]);
    assert_eq!(3, result);
}

#[test]
fn test2() {
    let result = Solution::num_times_all_blue(vec![3, 2, 4, 1, 5]);
    assert_eq!(2, result);
}

#[test]
fn test3() {
    let result = Solution::num_times_all_blue(vec![4, 1, 2, 3]);
    assert_eq!(1, result);
}

#[test]
fn test4() {
    let result = Solution::num_times_all_blue(vec![2, 1, 4, 3, 6, 5]);
    assert_eq!(3, result);
}

#[test]
fn test5() {
    let result = Solution::num_times_all_blue(vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(6, result);
}
