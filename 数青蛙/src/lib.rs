impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut frogs = vec![0; 5];

        fn find_frog(frogs: &mut Vec<usize>, c: char) -> bool {
            if c == 'c' && frogs[0] > 0 {
                frogs[0] -= 1;
                frogs[1] += 1;
            } else if c == 'r' && frogs[1] > 0 {
                frogs[1] -= 1;
                frogs[2] += 1;
            } else if c == 'o' && frogs[2] > 0 {
                frogs[2] -= 1;
                frogs[3] += 1;
            } else if c == 'a' && frogs[3] > 0 {
                frogs[3] -= 1;
                frogs[4] += 1;
            } else if c == 'k' && frogs[4] > 0 {
                frogs[4] -= 1;
                frogs[0] += 1;
            } else {
                return false;
            }
            true
        }

        for c in croak_of_frogs.chars() {
            if !find_frog(&mut frogs, c) {
                if c == 'c' {
                    frogs[1] += 1;
                } else {
                    return -1;
                }
            }
        }

        if frogs[1] != 0 || frogs[2] != 0 || frogs[3] != 0 || frogs[4] != 0 {
            return -1;
        }

        frogs[0] as i32
    }
}

pub struct Solution;
