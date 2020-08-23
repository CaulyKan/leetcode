impl Solution {
    pub fn split_array(nums: Vec<i32>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            let mut r = 0;
            let mut a = a;
            let mut b = b;
            if a < b {
                r = a;
                a = b;
                b = r;
            }
            while r != 0 {
                r = a % b;
                a = b;
                b = r;
            }
            a
        }

        if nums.len() == 0 {
            return 0;
        }
        let n = nums.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][i] = 1;
        }
        for len in 2..(n + 1) {
            for l in 0..(n - len + 1) {
                let r = l + len - 1;
                if gcd(nums[l], nums[r]) > 1 {
                    dp[l][r] = 1;
                } else {
                    dp[l][r] = len as i32;
                    for c in l..r {
                        dp[l][r] = std::cmp::min(dp[l][r], dp[l][c] + dp[c + 1][r]);
                    }
                }
            }
        }
        dp[0][n - 1]
    }
}
pub struct Solution;
