extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::max_size_slices(vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(10, result);
}

#[test]
fn test2() {
    let result = Solution::max_size_slices(vec![8, 9, 8, 6, 1, 1]);
    assert_eq!(16, result);
}

#[test]
fn test3() {
    let result = Solution::max_size_slices(vec![4, 1, 2, 5, 8, 3, 1, 9, 7]);
    assert_eq!(21, result);
}

#[test]
fn test4() {
    let result = Solution::max_size_slices(vec![3, 1, 2]);
    assert_eq!(3, result);
}

#[test]
fn test5() {
    let result = Solution::max_size_slices(vec![
        83, 41, 72, 88, 74, 72, 10, 23, 63, 27, 91, 31, 5, 84, 13, 38, 33, 22, 73, 1, 73, 82, 75,
        59, 90, 57, 28, 73, 92, 48, 61, 85, 45, 24, 63, 62, 65, 38, 80, 10, 63, 17, 94, 80, 85, 9,
        86, 38, 48, 27, 22, 55, 65, 68, 2, 89, 73, 44, 82, 55, 54, 79, 69, 21, 97, 55, 46, 68, 91,
        65, 97, 65, 80, 47, 92, 39, 33, 32, 31, 25, 36, 49, 89, 27, 78, 39, 86, 36, 28, 43, 35, 25,
        77, 71, 25, 32, 62, 10, 74, 42, 62, 30, 54, 11, 72, 88, 80, 42, 98, 86, 81,
    ]);
    assert_eq!(2996, result);
}
