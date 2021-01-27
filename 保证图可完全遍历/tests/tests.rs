extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::max_num_edges_to_remove(
        4,
        vec![
            vec![3, 1, 2],
            vec![3, 2, 3],
            vec![1, 1, 3],
            vec![1, 2, 4],
            vec![1, 1, 2],
            vec![2, 3, 4],
        ],
    );
    assert_eq!(2, result);
}

#[test]
fn test2() {
    let result = Solution::max_num_edges_to_remove(
        4,
        vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]],
    );
    assert_eq!(0, result);
}
