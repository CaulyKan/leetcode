pub struct Solution;
impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        fn count(n: i64) -> i64 {
            (1 + n) * n / 2
        }
        let mut result: i64 = 0;
        let chars: Vec<char> = s.chars().collect();
        let mut current = 'Z';
        let mut currentn = 0;
        for i in 0..s.len() {
            if chars[i] != current {
                result += count(currentn as i64);
                current = chars[i];
                currentn = 1;
            } else {
                currentn += 1;
            }
        }
        result += count(currentn as i64);

        (result % (1000000000 + 7)) as i32
    }
}
