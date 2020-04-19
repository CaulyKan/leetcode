extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::min_start_value(vec![-3, 2 - 3, 4, 2]);
    assert_eq!(5, result);
}
