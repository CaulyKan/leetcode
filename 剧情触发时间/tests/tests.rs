extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::get_trigger_time(
        vec![vec![0, 4, 5], vec![4, 8, 8], vec![8, 6, 1], vec![10, 10, 0]],
        vec![
            vec![12, 11, 16],
            vec![20, 2, 6],
            vec![9, 2, 6],
            vec![10, 18, 3],
            vec![8, 14, 9],
        ],
    );
    assert_eq!(vec![-1, 4, 3, 3, 3], result);
}

#[test]
fn test2() {
    let result = Solution::get_trigger_time(vec![vec![1, 1, 1]], vec![vec![0, 0, 0]]);
    assert_eq!(vec![0], result);
}
