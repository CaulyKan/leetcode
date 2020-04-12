extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::string_matching(vec![
        "mass".to_string(),
        "mast".to_string(),
        "as".to_string(),
        "hero".to_string(),
        "superhero".to_string(),
    ]);
    assert_eq!(vec!["as".to_string(), "hero".to_string()], result);
}
