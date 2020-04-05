extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::longest_diverse_string(1, 1, 7);
    assert_eq!("ccaccbcc", result);
}

#[test]
fn test2() {
    let result = Solution::longest_diverse_string(2, 2, 1);
    assert_eq!("aabbc", result);
}

#[test]
fn test3() {
    let result = Solution::longest_diverse_string(7, 1, 0);
    assert_eq!("aabaa", result);
}

#[test]
fn test4() {
    let result = Solution::longest_diverse_string(0, 8, 11);
    assert_eq!("ccbccbbccbbccbbccbc", result);
}
