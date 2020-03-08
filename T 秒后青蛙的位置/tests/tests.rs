extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::frog_position(
        7,
        vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 7],
            vec![2, 4],
            vec![2, 6],
            vec![3, 5],
        ],
        2,
        4,
    );
    assert_eq!(1.0 / 6.0, result);
}

#[test]
fn test2() {
    let result = Solution::frog_position(
        7,
        vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 7],
            vec![2, 4],
            vec![2, 6],
            vec![3, 5],
        ],
        20,
        6,
    );
    assert_eq!(1.0 / 6.0, result);
}

#[test]
fn test3() {
    let result = Solution::frog_position(3, vec![vec![2, 1], vec![3, 2]], 1, 2);
    assert_eq!(1.0, result);
}
