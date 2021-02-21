pub struct Solution;
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut c1 = 0;
        let mut c2 = 0;

        let mut flag = '0';
        for c in s.chars() {
            if c != flag {
                c1 += 1;
            }
            flag = if flag == '0' { '1' } else { '0' };
        }

        let mut flag = '1';
        for c in s.chars() {
            if c != flag {
                c2 += 1;
            }
            flag = if flag == '0' { '1' } else { '0' };
        }

        std::cmp::min(c1, c2)
    }
}
