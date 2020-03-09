/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start
#[derive(Debug)]
struct T {
    cont: usize,
    non_cont: Vec<usize>,
}

impl T {
    pub fn new() -> T {
        T {
            cont: 1,
            non_cont: Vec::new(),
        }
    }
    pub fn size(&self) -> usize {
        std::cmp::max(self.cont, *self.non_cont.last().unwrap_or(&0))
    }
}

impl Solution {
    pub fn longest_palindrome(s2: String) -> String {
        if s2 == "" {
            return String::from("");
        }
        if s2.len() == 1 {
            return s2;
        }
        let mut last = T::new();
        let s: Vec<char> = s2.chars().collect();
        let mut max = 0;
        let mut max_idx = 0;
        for i in 1..s.len() {
            let mut current = T::new();
            if s[i] == s[i - 1] {
                current.cont = last.cont + 1;
            } else {
                current.cont = 1;
            }

            if i > last.cont && s[i - last.cont - 1] == s[i] {
                current.non_cont.push(last.cont + 2);
            }

            for lastnc in &last.non_cont {
                if i > *lastnc && s[i - lastnc - 1] == s[i] {
                    current.non_cont.push(lastnc + 2);
                }
            }

            let m = current.size();
            if m > max {
                max = m;
                max_idx = i;
            }
            last = current;
        }
        s2[(max_idx + 1 - max)..max_idx + 1].to_string()
    }
}
// @lc code=end
pub struct Solution;
