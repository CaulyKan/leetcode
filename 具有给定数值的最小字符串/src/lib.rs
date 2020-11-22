impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let x = (26 * n - k) / 25;
        let mut v = vec!['a'; x as usize];
        let y = if n - x - 1 == 0 { 0 } else { (k - x - 1) / 26 };
        let m = k - x - y * 26;
        v.push(('a' as i32 + m - 1) as u8 as char);
        for _ in 0..y {
            v.push('z');
        }

        v.iter().collect()
    }
}

pub struct Solution;
