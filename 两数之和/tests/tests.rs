extern crate two_sum;

#[test]
fn test1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum::Solution::two_sum(nums, target);
    assert_eq!(vec![0, 1], result);
}
