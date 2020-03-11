/*
 * @lc app=leetcode.cn id=1013 lang=rust
 *
 * [1013] 将数组分成和相等的三个部分
 */

// @lc code=start
impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let mut map = std::collections::HashMap::<i64, Vec<usize>>::new();
        let mut sum: i64 = 0;
        for i in 0..a.len() - 1 {
            sum += a[i] as i64;
            if map.contains_key(&sum) {
                map.get_mut(&sum).unwrap().push(i);
            } else {
                map.insert(sum, vec![i]);
            }
        }
        sum += *a.last().unwrap() as i64;
        if sum % 3 == 0 {
            let v = sum / 3;
            let v2 = v * 2;
            if map.contains_key(&v) && map.contains_key(&v2) {
                if map[&v].first().unwrap() < map[&v2].last().unwrap() {
                    return true;
                }
            }
        }
        false
    }
}
// @lc code=end
pub struct Solution;
