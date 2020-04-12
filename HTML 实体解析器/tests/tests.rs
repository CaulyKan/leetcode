extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result =
        Solution::entity_parser("x &gt; y &amp;&amp; x &lt; y is always false".to_string());
    assert_eq!("x > y && x < y is always false".to_string(), result);
}
