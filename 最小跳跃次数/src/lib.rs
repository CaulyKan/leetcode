impl Solution {
    pub fn min_jump(jump: Vec<i32>) -> i32 {
        let len = jump.len() as i32;
        let mut dp: Vec<i32> = jump
            .iter()
            .enumerate()
            .map(|(pos, val)| if pos as i32 + val >= len { 1 } else { -1 })
            .collect();

        loop {
            let mut min = -1;
            let mut last = jump.len();
            for (pos, val) in jump.iter().enumerate().take(last) {
                if dp[pos] == -1 {
                    let right = dp[pos + *val as usize];
                    if min > 0 && right > 0 {
                        dp[pos] = std::cmp::min(min, right) + 1;
                    } else if min > 0 {
                        dp[pos] = min + 1;
                    } else if right > 0 {
                        dp[pos] = right + 1;
                    } else {
                        last = pos + 1;
                    }
                } else {
                    min = if min == -1 {
                        dp[pos]
                    } else {
                        std::cmp::min(min, dp[pos])
                    };
                }
            }

            if dp[0] != -1 {
                return dp[0];
            }
        }
    }
}

pub struct Solution;
