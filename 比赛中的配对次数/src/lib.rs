pub struct Solution;

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut i = n;
        let mut n = 0;
        while i > 1 {
            if i % 2 == 0 {
                n += i / 2;
                i = i / 2;
            } else {
                n += (i - 1) / 2;
                i = (i - 1) / 2 + 1;
            }
        }
        n
    }
}
