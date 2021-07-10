pub struct Solution;
use std::collections::*;
impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut max = i32::min_value();
        let mut origin_sum: i64 = 0;
        let mut max2 = 0;
        let mut max1 = 0;
        for i in 0..nums1.len() {
            let a = (nums1[i] - nums2[i]).abs();
            if a > max {
                max = a;
                max1 = nums1[i];
                max2 = nums2[i];
            }
            origin_sum += a as i64;
        }
        if origin_sum == 0 {
            return 0;
        }
        println!("{} {}", max, origin_sum);

        let mut set1: HashSet<i32> = HashSet::new();
        for i in nums1 {
            set1.insert(i);
        }

        for i in 0..max + 1 {
            if set1.contains(&(max2 + i)) || set1.contains(&(max2 - i)) {
                println!("{} {} {} {}\n", i, max, max1, max2);
                return ((origin_sum - (max - i) as i64) % 1000000007) as i32;
            }
        }

        0
    }
}
