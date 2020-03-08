extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::find_the_longest_substring("eleetminicoworoep".to_string());
    assert_eq!(13, result);
}

#[test]
fn test2() {
    let result = Solution::find_the_longest_substring("leetcodeisgreat".to_string());
    assert_eq!(5, result);
}

#[test]
fn test3() {
    let result = Solution::find_the_longest_substring("bcbcbc".to_string());
    assert_eq!(6, result);
}

#[test]
fn test4() {
    let result = Solution::find_the_longest_substring("a".to_string());
    assert_eq!(0, result);
}
