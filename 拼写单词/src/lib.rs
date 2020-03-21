/*
 * @lc app=leetcode.cn id=1160 lang=rust
 *
 * [1160] 拼写单词
 */

// @lc code=start
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut map = [0; 26];
        for c in chars.bytes() {
            map[(c - 'a' as u8) as usize] += 1;
        }

        let mut result: i32 = 0;

        'outer: for word in words {
            let mut wmap = [0; 26];
            for c in word.bytes() {
                wmap[(c - 'a' as u8) as usize] += 1;
                if wmap[(c - 'a' as u8) as usize] > map[(c - 'a' as u8) as usize] {
                    continue 'outer;
                }
            }
            result += word.len() as i32;
        }

        result
    }
}
// @lc code=end

pub struct Solution;
