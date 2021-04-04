extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::count_restricted_paths(
        5,
        vec![
            vec![1, 2, 3],
            vec![1, 3, 3],
            vec![2, 3, 1],
            vec![1, 4, 2],
            vec![5, 2, 2],
            vec![3, 5, 1],
            vec![5, 4, 10],
        ],
    );
    assert_eq!(3, result);
}

#[test]
fn test2() {
    let result = Solution::count_restricted_paths(
        7,
        vec![
            vec![1, 3, 1],
            vec![4, 1, 2],
            vec![7, 3, 4],
            vec![2, 5, 3],
            vec![5, 6, 1],
            vec![6, 7, 2],
            vec![7, 5, 3],
            vec![2, 6, 4],
        ],
    );
    assert_eq!(1, result);
}

#[test]
fn test3() {
    let result = Solution::count_restricted_paths(
        5,
        vec![
            vec![2, 1, 77061],
            vec![4, 3, 77398],
            vec![1, 4, 22974],
            vec![4, 2, 45825],
            vec![2, 3, 56161],
            vec![2, 5, 79345],
            vec![1, 3, 91496],
            vec![3, 5, 73435],
        ],
    );
    assert_eq!(6, result);
}

#[test]
fn test4() {
    let result = Solution::count_restricted_paths(
        10,
        vec![
            vec![9, 10, 8],
            vec![9, 6, 5],
            vec![1, 5, 9],
            vec![6, 8, 10],
            vec![1, 8, 1],
            vec![8, 10, 7],
            vec![10, 7, 9],
            vec![5, 7, 3],
            vec![4, 2, 9],
            vec![2, 3, 9],
            vec![3, 10, 4],
            vec![1, 4, 7],
            vec![7, 6, 1],
            vec![3, 9, 8],
            vec![9, 1, 6],
            vec![4, 7, 10],
            vec![9, 4, 9],
        ],
    );
    assert_eq!(6, result);
}
