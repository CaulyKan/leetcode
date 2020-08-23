extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::reverse_pairs(vec![7, 5, 6, 4]);
    assert_eq!(5, result);
}
