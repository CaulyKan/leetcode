extern crate leetcode_utils;
use leetcode_utils::union_find::*;

#[test]
fn test1() {
    let mut uf = UnionFind::new(4);
    uf.union_node(1, 2);
    uf.union_node(2, 3);
    assert_eq!(2, uf.union_count());
    assert_eq!(1, uf.find(3));
}
