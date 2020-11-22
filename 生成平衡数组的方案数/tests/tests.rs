extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::ways_to_make_fair(vec![2, 1, 6, 4]);
    assert_eq!(1, result);
}

#[test]
fn test2() {
    let result = Solution::ways_to_make_fair(vec![1, 1, 1]);
    assert_eq!(3, result);
}

#[test]
fn test3() {
    let result = Solution::ways_to_make_fair(vec![1, 2, 3]);
    assert_eq!(0, result);
}
