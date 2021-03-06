pub struct Solution;
use cauly_rust_leetcode_utils::define_dp;
use cauly_rust_leetcode_utils::dp::*;

struct State {
    position: usize,
    step: usize,
}
define_dp! {State, position:2001, step:1200}

use std::collections::*;
impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let target = *stones.last().unwrap();
        println!("{}", stones.len());
        if stones.len() == 2 {
            return stones[1] == 1;
        }
        let mut map: HashMap<i32, i32> = std::collections::HashMap::new();
        for (index, stone) in stones.iter().enumerate() {
            map.insert(*stone, index as i32);
        }

        let mut dp = DP::new::<State>(false);

        let mut current: Vec<(i32, i32)> = vec![(1, 1)];
        loop {
            let mut next = Vec::new();
            for &(pos, size) in &current {
                let t = stones[pos as usize] + size;
                if map.contains_key(&t) {
                    let idx = map[&(t)];
                    if t == target {
                        return true;
                    }
                    if !dp.get(&State {
                        position: idx as usize,
                        step: size as usize,
                    }) {
                        next.push((map[&t], size));
                        *dp.get_mut(&State {
                            position: idx as usize,
                            step: size as usize,
                        }) = true;
                    }
                }
                if map.contains_key(&(t + 1)) {
                    let idx = map[&(t + 1)];
                    if t + 1 == target {
                        return true;
                    }
                    if !dp.get(&State {
                        position: idx as usize,
                        step: size as usize + 1,
                    }) {
                        next.push((idx, size + 1));
                        *dp.get_mut(&State {
                            position: idx as usize,
                            step: size as usize + 1,
                        }) = true;
                    }
                }
                if size > 1 && map.contains_key(&(t - 1)) {
                    let idx = map[&(t - 1)];
                    if t - 1 == target {
                        return true;
                    }
                    if !dp.get(&State {
                        position: idx as usize,
                        step: size as usize - 1,
                    }) {
                        next.push((idx, size - 1));
                        *dp.get_mut(&State {
                            position: idx as usize,
                            step: size as usize - 1,
                        }) = true;
                    }
                }
            }
            //println!("{:?}", next);
            if next.len() == 0 {
                break;
            }
            current = next;
        }

        false
    }
}
