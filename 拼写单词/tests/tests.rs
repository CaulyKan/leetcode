extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::count_characters(
        vec![
            "cat".to_string(),
            "bt".to_string(),
            "hat".to_string(),
            "tree".to_string(),
        ],
        "atach".to_string(),
    );
    assert_eq!(6, result);
}

#[test]
fn test2() {
    let result = Solution::count_characters(
        vec![
            "hello".to_string(),
            "world".to_string(),
            "leetcode".to_string(),
        ],
        "welldonehoneyr".to_string(),
    );
    assert_eq!(10, result);
}
