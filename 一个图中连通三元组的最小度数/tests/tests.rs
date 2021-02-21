extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::min_trio_degree(
        6,
        vec![
            vec![1, 2],
            vec![1, 3],
            vec![3, 2],
            vec![4, 1],
            vec![5, 2],
            vec![3, 6],
        ],
    );
    assert_eq!(3, result);
}
