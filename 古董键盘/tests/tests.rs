extern crate leetcode;
use leetcode::Solution;

// FAILED

#[test]
fn test1() {
    let result = Solution::keyboard2(1, 1);
    assert_eq!(26, result);
}

#[test]
fn test2() {
    let result = Solution::keyboard2(1, 2);
    assert_eq!(650, result);
}

#[test]
fn test3() {
    let result1 = Solution::keyboard(2, 3);
    let result2 = Solution::keyboard2(2, 3);
    assert_eq!(result1, result2);
}

#[test]
fn test4() {
    let result1 = Solution::keyboard(1, 3);
    let result2 = Solution::keyboard2(1, 3);
    assert_eq!(result1, result2);
}

#[test]
fn test5() {
    let result1 = Solution::keyboard(2, 4);
    let result2 = Solution::keyboard2(2, 4);
    assert_eq!(result1, result2);
}

#[test]
fn test6() {
    let result1 = Solution::keyboard(5, 26 * 3);
    //let result2 = Solution::keyboard2(5, 3 * 26);
    assert_eq!(result1, 0);
}
