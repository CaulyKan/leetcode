extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::minimum_size(vec![9], 2);
    assert_eq!(3, result);
}

#[test]
fn test2() {
    let result = Solution::minimum_size(vec![2, 4, 8, 2], 4);
    assert_eq!(2, result);
}

#[test]
fn test3() {
    let result = Solution::minimum_size(vec![7, 17], 2);
    assert_eq!(7, result);
}

#[test]
fn test4() {
    let result = Solution::minimum_size(
        vec![
            431, 922, 158, 60, 192, 14, 788, 146, 788, 775, 772, 792, 68, 143, 376, 375, 877, 516,
            595, 82, 56, 704, 160, 403, 713, 504, 67, 332, 26,
        ],
        80,
    );
    assert_eq!(129, result);
}
