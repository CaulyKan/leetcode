pub struct Solution;
use std::cmp::min;
impl Solution {
    pub fn min_characters(a: String, b: String) -> i32 {
        let mut amap = vec![0; 26];
        let mut bmap = vec![0; 26];
        let mut result = i32::max_value();
        for c in a.chars() {
            amap[(c as i32 - 97) as usize] += 1;
        }
        for c in b.chars() {
            bmap[(c as i32 - 97) as usize] += 1;
        }

        for i in 1..26 {
            let target = i as usize;

            let mut c_count = 0;
            let mut b_count = 0;
            let mut a_count = 0;
            // 全部改成target
            for k in 0..amap.len() {
                let v = amap[k];
                if k != target {
                    c_count += v;
                }
                if k >= target {
                    b_count += v;
                }
                if k < target {
                    a_count += v;
                }
            }
            for k in 0..bmap.len() {
                let v = bmap[k];
                if k != target {
                    c_count += v;
                }
                if k < target {
                    b_count += v;
                }
                if k >= target {
                    a_count += v;
                }
            }

            result = min(result, a_count);
            result = min(result, b_count);
            result = min(result, c_count);
        }

        let mut d_count = 0;
        for k in 0..amap.len() {
            let v = amap[k];
            if k != 0 {
                d_count += v;
            }
        }
        for k in 0..bmap.len() {
            let v = bmap[k];
            if k != 0 {
                d_count += v;
            }
        }
        result = min(result, d_count);

        result
    }
}
