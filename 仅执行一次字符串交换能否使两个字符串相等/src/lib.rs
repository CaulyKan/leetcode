pub struct Solution;
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1 == s2 {
            return true;
        }
        if s1.len() == 1 {
            return false;
        }

        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();

        let mut diff1 = 'a';
        let mut diff2 = 'a';
        let mut diffcount = 0;

        for i in 0..s1.len() {
            if s1[i] != s2[i] {
                if diffcount == 1 {
                    if diff1 != s2[i] || diff2 != s1[i] {
                        return false;
                    }
                    diffcount += 1;
                } else if diffcount > 1 {
                    return false;
                } else {
                    diff1 = s1[i];
                    diff2 = s2[i];
                    diffcount += 1;
                }
            }
        }

        if diffcount == 1 {
            false
        } else {
            true
        }
    }
}
