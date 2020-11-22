impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        if word1.join("") == word2.join("") {
            true
        } else {
            false
        }
    }
}

pub struct Solution;
