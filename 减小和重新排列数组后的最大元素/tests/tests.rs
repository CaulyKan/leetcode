extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 2, 3, 4, 5]);
    assert_eq!(5, result);
}

#[test]
fn test2() {
    let result = Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 2, 2, 2, 1]);
    assert_eq!(2, result);
}

#[test]
fn test3() {
    let result = Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 100, 1000]);
    assert_eq!(3, result);
}

#[test]
fn test4() {
    let result = Solution::maximum_element_after_decrementing_and_rearranging(vec![2, 2, 2, 2]);
    assert_eq!(2, result);
}
