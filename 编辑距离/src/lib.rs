pub struct Solution;
use cauly_rust_leetcode_utils::define_dp;
use cauly_rust_leetcode_utils::dp::*;
use std::cmp::*;

struct State {
    pos1: usize,
    pos2: usize,
}
define_dp!(State, pos1:501, pos2:501);

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = DP::new::<State>(0 as i32);
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();

        for i in 0..word1.len() + 1 {
            *dp.get_mut(&State { pos1: i, pos2: 0 }) = i as i32;
        }
        for i in 0..word2.len() + 1 {
            *dp.get_mut(&State { pos1: 0, pos2: i }) = i as i32;
        }
        for j in 1..word2.len() + 1 {
            for i in 1..word1.len() + 1 {
                *dp.get_mut(&State { pos1: i, pos2: j }) = min(
                    dp.get(&State {
                        pos1: i,
                        pos2: j - 1,
                    }) + 1,
                    min(
                        dp.get(&State {
                            pos1: i - 1,
                            pos2: j - 1,
                        }) + if word1[i - 1] == word2[j - 1] { 0 } else { 1 },
                        dp.get(&State {
                            pos1: i - 1,
                            pos2: j,
                        }) + 1,
                    ),
                );
            }
        }
        dp.get(&State {
            pos1: word1.len(),
            pos2: word2.len(),
        })
    }
}
