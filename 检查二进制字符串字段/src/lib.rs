pub struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let mut a1 = false;
        let mut a2 = true;
        for i in 0..s.len() {
            if s[i] == '1' {
                a1 = true;
            }
            if a1 && s[i] == '0' {
                a2 = false;
            }
            if s[i] == '1' && !a2 {
                return false;
            }
        }

        true
    }
}
