impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        fn is_valid(string: &str) -> bool {
            let mut v = [0, 0, 0, 0, 0];
            for c in string.chars() {
                match c {
                    'a' => v[0] += 1,
                    'e' => v[1] += 1,
                    'i' => v[2] += 1,
                    'o' => v[3] += 1,
                    'u' => v[4] += 1,
                    _ => (),
                };
            }
            v[0] % 2 == 0 && v[1] % 2 == 0 && v[2] % 2 == 0 && v[3] % 2 == 0 && v[4] % 2 == 0
        }
        for len in (1..s.len() + 1).rev() {
            for start in 0..s.len() - len + 1 {
                if is_valid(&s[start..start + len]) {
                    return len as i32;
                }
            }
        }
        0
    }
}
pub struct Solution;
