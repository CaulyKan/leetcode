extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::num_ways(
        5,
        vec![
            vec![0, 2],
            vec![2, 1],
            vec![3, 4],
            vec![2, 3],
            vec![1, 4],
            vec![2, 0],
            vec![0, 4],
        ],
        3,
    );
    assert_eq!(3, result);
}

#[test]
fn test2() {
    let result = Solution::num_ways(5, vec![vec![0, 2], vec![2, 1]], 3);
    assert_eq!(0, result);
}
