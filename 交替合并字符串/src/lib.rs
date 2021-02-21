pub struct Solution;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = "".to_string();
        let w1: Vec<char> = word1.chars().collect();
        let w2: Vec<char> = word2.chars().collect();
        let mut i = 0;

        loop {
            if i < w1.len() {
                result.push_str(w1[i].to_string().as_str());
            }
            if i < w2.len() {
                result.push_str(w2[i].to_string().as_str());
            }
            i += 1;
            if result.len() == w1.len() + w2.len() {
                break;
            }
        }

        result
    }
}
