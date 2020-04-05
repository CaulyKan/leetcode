extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::num_steps("1101".to_string());
    assert_eq!(6, result);
}

#[test]
fn test2() {
    let result = Solution::num_steps("1".to_string());
    assert_eq!(0, result);
}

#[test]
fn test3() {
    let result = Solution::num_steps("1000101011101101001100011010111111011111010110100001101110110100000011100100010010101110101010010110000111001011000011000000011010101011100111001001110010000011101".to_string());
}

#[test]
fn test4() {
    let result = Solution::num_steps(
        "1111110011101010110011100100101110010100101110111010111110110010".to_string(),
    );
}

#[test]
fn test5() {
    let result = Solution::num_steps(
        "100100001010010101101000111100100111101111000111111010010001100001000100011001"
            .to_string(),
    );
    assert_eq!(120, result);
}
