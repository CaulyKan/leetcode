impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }

        let mut map = vec![0; 26];
        for c in s.bytes() {
            map[(c - ('a' as u8)) as usize] += 1;
        }

        let mut count = 0;
        for i in 0..26 {
            if map[i] % 2 != 0 {
                count += 1;
            }
        }

        println!("{}", count);
        count <= k
    }
}

pub struct Solution;
