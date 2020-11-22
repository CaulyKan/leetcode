extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::array_strings_are_equal(
        vec!["Foo".to_string(), "Bar".to_string()],
        vec!["FooB".to_string(), "ar".to_string()],
    );
    assert_eq!(true, result);
}
