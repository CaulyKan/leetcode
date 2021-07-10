extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::sum_of_floored_pairs(vec![2, 5, 9]);
    assert_eq!(10, result);
}
