pub struct Solution;
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let index = if rule_key == "type" {
            0
        } else if rule_key == "color" {
            1
        } else {
            2
        };
        let mut result = 0;
        for i in &items {
            if i[index] == rule_value {
                result += 1;
            }
        }
        result
    }
}
