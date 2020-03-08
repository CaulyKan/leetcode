use std::collections::BTreeMap;
impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut map = [0; 256];
        let mut result = String::new();
        for c in s.chars() {
            map[c as usize] += 1;
        }

        if s.len() == 0 || s.len() == 1 {
            return s;
        }

        while result.len() < s.len() {
            for i in 0..256 {
                if map[i] > 0 {
                    result.push(i as u8 as char);
                    map[i] -= 1;
                }
            }
            for i in (0..256).rev() {
                if map[i] > 0 {
                    result.push(i as u8 as char);
                    map[i] -= 1;
                }
            }
        }

        return result;
    }
}
pub struct Solution;
