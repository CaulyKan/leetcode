pub struct Solution;
impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let sum1: i32 = nums1.iter().sum();
        let sum2: i32 = nums2.iter().sum();

        if sum1 > sum2 {
            let mut target = sum1 - sum2;
            let mut count = 0;
            let mut s: Vec<i32> = nums1
                .iter()
                .map(|x| x - 1)
                .chain(nums2.iter().map(|x| 6 - x))
                .collect();
            s.sort();
            for i in s.iter().rev() {
                target -= i;
                count += 1;
                if target <= 0 {
                    return count;
                }
            }
        } else if sum1 < sum2 {
            let mut target = sum2 - sum1;
            let mut count = 0;
            let mut s: Vec<i32> = nums1
                .iter()
                .map(|x| 6 - x)
                .chain(nums2.iter().map(|x| x - 1))
                .collect();
            s.sort();
            for i in s.iter().rev() {
                target -= i;
                count += 1;
                if target <= 0 {
                    return count;
                }
            }
        } else {
            return 0;
        }
        -1
    }
}
