impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        for (i, s1) in words.iter().enumerate() {
            for (j, s2) in words.iter().enumerate() {
                if i != j && s2.contains(s1) {
                    result.push(s1.clone());
                    break;
                }
            }
        }

        result
    }
}

pub struct Solution;
