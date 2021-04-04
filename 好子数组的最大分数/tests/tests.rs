extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::maximum_score(vec![1, 4, 3, 7, 4, 5], 3);
    assert_eq!(15, result);
}
