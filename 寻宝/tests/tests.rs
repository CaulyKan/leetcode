extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::minimal_steps(vec![
        "S#O".to_string(),
        "M..".to_string(),
        "M.T".to_string(),
    ]);
    assert_eq!(16, result);
}

#[test]
fn test2() {
    let result = Solution::minimal_steps(vec![
        "S#O".to_string(),
        "M.#".to_string(),
        "M.T".to_string(),
    ]);
    assert_eq!(-1, result);
}

#[test]
fn test3() {
    let result = Solution::minimal_steps(vec![
        "S#O".to_string(),
        "M.T".to_string(),
        "M..".to_string(),
    ]);
    assert_eq!(17, result);
}

#[test]
fn test4() {
    let result = Solution::minimal_steps(vec!["ST".to_string()]);
    assert_eq!(1, result);
}

#[test]
fn test5() {
    let result = Solution::minimal_steps(vec!["S#O".to_string(), "M.T".to_string()]);
    assert_eq!(9, result);
}

#[test]
fn test6() {
    let result = Solution::minimal_steps(vec![
        "SOO".to_string(),
        "MT#".to_string(),
        "M#O".to_string(),
    ]);
    assert_eq!(10, result);
}

#[test]
fn test7() {
    let result = Solution::minimal_steps(vec![
        "SOO".to_string(),
        "M.#".to_string(),
        "M#T".to_string(),
    ]);
    assert_eq!(-1, result);
}

#[test]
fn test8() {
    let result = Solution::minimal_steps(vec!["S#T".to_string()]);
    assert_eq!(-1, result);
}

#[test]
fn test9() {
    let result = Solution::minimal_steps(vec!["OSMT".to_string()]);
    assert_eq!(4, result);
}

#[test]
fn test10() {
    let result = Solution::minimal_steps(vec![
        "O".to_string(),
        "S".to_string(),
        "M".to_string(),
        "T".to_string(),
    ]);
    assert_eq!(4, result);
}

#[test]
fn test11() {
    let result = Solution::minimal_steps(vec!["O#SMMMMMMMMMMMMT".to_string()]);
    assert_eq!(-1, result);
}

#[test]
fn test13() {
    let result = Solution::minimal_steps(vec!["OSMOMT".to_string()]);
    assert_eq!(6, result);
}

#[test]
fn test14() {
    let result = Solution::minimal_steps(vec![
        "STM.............".to_string(),
        "#########O#####.".to_string(),
        ".O.......M......".to_string(),
    ]);
    assert_eq!(21, result);
}

#[test]
fn test15() {
    let result = Solution::minimal_steps(vec![
        "OOOM".to_string(),
        "MOMO".to_string(),
        "MOTS".to_string(),
        "MMOO".to_string(),
    ]);
    assert_eq!(17, result);
}
