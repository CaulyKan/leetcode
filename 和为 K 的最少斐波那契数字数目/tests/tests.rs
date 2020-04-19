extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::find_min_fibonacci_numbers(19);
    assert_eq!(3, result);
}
