extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::num_similar_groups(vec![
        "tars".to_string(),
        "rats".to_string(),
        "arts".to_string(),
        "star".to_string(),
    ]);
    assert_eq!(2, result);
}
