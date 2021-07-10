extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::max_distance(vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5]);
    assert_eq!(2, result);
}
