pub struct Solution;
impl Solution {
    pub fn merge(a: &mut Vec<i32>, m: i32, b: &mut Vec<i32>, n: i32) {
        let mut last_a = m - 1;
        let mut last_b = n - 1;
        let mut last = m + n - 1;
        if n == 0 {
            return;
        }
        if m == 0 {
            for i in 0..n as usize {
                a[i] = b[i];
            }
            return;
        }
        while last_a >= 0 && last_b >= 0 {
            if a[last_a as usize] >= b[last_b as usize] {
                a[last as usize] = a[last_a as usize];
                last_a -= 1;
                last -= 1;
            } else {
                a[last as usize] = b[last_b as usize];
                last_b -= 1;
                last -= 1;
            }
        }
        if last_a < 0 {
            for i in 0..(last_b + 1) as usize {
                a[i] = b[i];
            }
        }
    }
}
