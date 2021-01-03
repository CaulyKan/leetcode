pub struct Solution;
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let c: Vec<char> = s.chars().collect();
        let a = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut l = 0;
        let mut r = 0;
        for i in 0..s.len() / 2 {
            if a.contains(&c[i]) {
                l += 1;
            }
        }
        for i in s.len() / 2..s.len() {
            if a.contains(&c[i]) {
                r += 1;
            }
        }
        l == r
    }
}
