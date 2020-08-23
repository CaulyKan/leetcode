extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::max_coins(vec![2, 4, 1, 2, 7, 8]);
    assert_eq!(9, result);
}
