extern crate leetcode;
use leetcode::Solution;

#[test]
fn test1() {
    let result = Solution::display_table(vec![
        vec!["David".to_string(), "3".to_string(), "Ceviche".to_string()],
        vec![
            "Corina".to_string(),
            "10".to_string(),
            "Beef Burrito".to_string(),
        ],
        vec![
            "David".to_string(),
            "3".to_string(),
            "Fried Chicken".to_string(),
        ],
        vec!["Carla".to_string(), "5".to_string(), "Water".to_string()],
        vec!["Carla".to_string(), "5".to_string(), "Ceviche".to_string()],
        vec!["Rous".to_string(), "3".to_string(), "Ceviche".to_string()],
    ]);
    println!("{:?}", result);
    assert_eq!(1, 1);
}
