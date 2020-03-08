impl Solution {
    pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut result = 0;
        let mut right: usize = 0;
        let mut list = [0; 50000];
        for k in light {
            right = std::cmp::max(right, k as usize);
            list[(k - 1) as usize] = 1;
            if (k - 1) as usize == left {
                let mut flag = true;
                for i in left..right {
                    if list[i] == 0 {
                        left = i;
                        flag = false;
                        break;
                    }
                }
                if flag {
                    left = right;
                    result += 1;
                }
            }
        }
        result
    }
}

pub struct Solution;
