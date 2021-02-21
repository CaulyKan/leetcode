pub struct Solution;
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();
        let mut multipliers = multipliers;
        multipliers.reverse();
        let mut dp = vec![0; n];
        let last = n - m;
        for i in last + 1..n {
            let mut new = vec![0; n];
            for start in 0..n {
                let end = start + i - 1;
                let index = i - 1 - last;
                if end >= n {
                    continue;
                }
                new[start] = if start == end {
                    nums[start] * multipliers[index]
                } else {
                    std::cmp::max(
                        dp[start + 1] + nums[start] * multipliers[index],
                        dp[start] + nums[end] * multipliers[index],
                    )
                }
            }
            dp = new;
        }

        dp[0]
    }
}
