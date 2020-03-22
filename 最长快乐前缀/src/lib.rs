impl Solution {
    pub fn longest_prefix(s: String) -> String {
        if s.len() <= 1 {
            return "".to_string();
        }

        let v: Vec<char> = s.chars().collect();
        let last = *v.last().unwrap();

        for i in (0..v.len() - 1).rev() {
            if v[i] == last {
                if v[0..i + 1] == v[v.len() - i - 1..v.len()] {
                    return v[0..i + 1].iter().collect::<String>();
                }
            }
        }

        "".to_string()
    }
}

pub struct Solution;
