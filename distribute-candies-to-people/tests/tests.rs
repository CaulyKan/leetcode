extern crate leetcode;

#[test]
fn test1() {
    let result = leetcode::Solution::distribute_candies(7, 4);
    assert_eq!(vec![1, 2, 3, 1], result);
}

#[test]
fn test2() {
    let result = leetcode::Solution::distribute_candies(10, 3);
    assert_eq!(vec![5, 2, 3], result);
}

#[test]
fn test3() {
    let result = leetcode::Solution::distribute_candies(10, 1);
    assert_eq!(vec![10], result);
}
