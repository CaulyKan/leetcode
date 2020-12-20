extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::reformat_number("1-23-45 6".to_string());
    assert_eq!("123-456".to_string(), result);
}

#[test]
fn test2() {
    let result = Solution::reformat_number("123 4-567".to_string());
    assert_eq!("123-45-67".to_string(), result);
}

#[test]
fn test3() {
    let result = Solution::reformat_number("123 4-5678".to_string());
    assert_eq!("123-456-78".to_string(), result);
}

#[test]
fn test4() {
    let result = Solution::reformat_number("12".to_string());
    assert_eq!("12".to_string(), result);
}

#[test]
fn test5() {
    let result = Solution::reformat_number("--17-5 229 35-39475 ".to_string());
    assert_eq!("175-229-353-94-75".to_string(), result);
}
