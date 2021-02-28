extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result =
        Solution::get_collision_times(vec![vec![1, 2], vec![2, 1], vec![4, 3], vec![7, 2]]);
    assert_eq!(vec![1.00000, -1.00000, 3.00000, -1.00000], result);
}
