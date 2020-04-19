impl Solution {
    pub fn reformat(s: String) -> String {
        let mut chars = vec![];
        let mut alphabets = vec![];
        for i in s.chars() {
            if i.is_alphabetic() {
                alphabets.push(i);
            } else {
                chars.push(i);
            }
        }

        if (chars.len() as i32 - alphabets.len() as i32).abs() > 1 {
            "".to_string()
        } else {
            let mut result = String::with_capacity(s.len());
            if chars.len() > alphabets.len() {
                for i in 0..alphabets.len() {
                    result.push(chars[i]);
                    result.push(alphabets[i]);
                }
                result.push(chars[chars.len() - 1]);
            } else if chars.len() < alphabets.len() {
                for i in 0..chars.len() {
                    result.push(alphabets[i]);
                    result.push(chars[i]);
                }
                result.push(alphabets[alphabets.len() - 1]);
            } else {
                for i in 0..chars.len() {
                    result.push(alphabets[i]);
                    result.push(chars[i]);
                }
            }
            result
        }
    }
}

pub struct Solution;
