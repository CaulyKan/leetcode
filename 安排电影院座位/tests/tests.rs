extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::max_number_of_families(
        3,
        vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 8],
            vec![2, 6],
            vec![3, 1],
            vec![3, 10],
        ],
    );
    assert_eq!(4, result);
}

#[test]
fn test2() {
    let result = Solution::max_number_of_families(2, vec![vec![2, 1], vec![1, 8], vec![2, 6]]);
    assert_eq!(2, result);
}

#[test]
fn test3() {
    let result =
        Solution::max_number_of_families(4, vec![vec![4, 3], vec![1, 4], vec![4, 6], vec![1, 7]]);
    assert_eq!(4, result);
}
