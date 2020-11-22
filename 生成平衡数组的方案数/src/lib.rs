impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let mut osum = 0;
        let mut esum = 0;
        let mut osumv = vec![];
        let mut esumv = vec![];
        for i in 0..nums.len() {
            if i % 2 == 0 {
                esum += nums[i];
                esumv.push(esum);
                osumv.push(osum);
            } else {
                osum += nums[i];
                esumv.push(esum);
                osumv.push(osum);
            }
        }

        let mut result = 0;

        for i in 0..nums.len() {
            let v = nums[i];
            let eremain = esum - esumv[i];
            let oremain = osum - osumv[i];
            if i % 2 == 0 {
                if esumv[i] - v + oremain == osumv[i] + eremain {
                    result += 1;
                }
            } else {
                if esumv[i] + oremain == osumv[i] - v + eremain {
                    result += 1;
                }
            }
        }

        result
    }
}

pub struct Solution;
