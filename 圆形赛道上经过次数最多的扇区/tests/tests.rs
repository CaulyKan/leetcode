extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::most_visited(4, vec![1, 3, 1, 2]);
    assert_eq!(vec![1, 2], result);
}

#[test]
fn test2() {
    let result = Solution::most_visited(2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2]);
    assert_eq!(vec![2], result);
}

#[test]
fn test3() {
    let result = Solution::most_visited(7, vec![1, 3, 5, 7]);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], result);
}
