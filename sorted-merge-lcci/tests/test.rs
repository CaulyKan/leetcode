extern crate sorted_merge_lcci;

#[test]
fn test1() {
    let mut a = vec![1, 2, 3, 0, 0, 0];
    let mut b = vec![2, 5, 6];
    sorted_merge_lcci::Solution::merge(&mut a, 3, &mut b, 3);
    assert_eq!(vec![1, 2, 2, 3, 5, 6], a);
}

#[test]
fn test2() {
    let mut a = vec![2, 0];
    let mut b = vec![1];
    sorted_merge_lcci::Solution::merge(&mut a, 1, &mut b, 1);
    assert_eq!(vec![1, 2], a);
}
