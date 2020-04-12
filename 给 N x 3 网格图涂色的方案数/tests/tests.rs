extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::num_of_ways(2);
    assert_eq!(54, result);
}

#[test]
fn test2() {
    let result = Solution::num_of_ways(5000);
    assert_eq!(30228214, result);
}
