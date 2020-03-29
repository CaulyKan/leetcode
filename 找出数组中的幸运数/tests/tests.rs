extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::find_lucky(vec![2, 2, 3, 4]);
    assert_eq!(2, result);
}
