pub struct Solution;

impl Solution {
    pub fn compress_string(s: String) -> String {
        let mut result = String::new();
        if s.len() <= 1 {
            return s;
        }

        let mut last = s.chars().next().unwrap();
        let mut last_length = 1;

        for c in s.chars().skip(1) {
            if c != last {
                if last_length > 0 {
                    result.push(last);
                    result.push_str(&last_length.to_string());
                }
                last = c;
                last_length = 1;
            } else {
                last_length += 1;
            }
        }
        if last_length > 0 {
            result.push(last);
            result.push_str(&last_length.to_string());
        }

        if result.len() >= s.len() {
            s
        } else {
            result
        }
    }
}
