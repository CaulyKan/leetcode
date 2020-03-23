extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3]);
    assert_eq!(true, result);
}
