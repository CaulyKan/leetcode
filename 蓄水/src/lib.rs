pub struct Solution;
impl Solution {
    pub fn store_water(bucket: Vec<i32>, vat: Vec<i32>) -> i32 {
        let mut result = i32::max_value();
        for pour_count in 0..10000 {
            let mut count: i64 = 0;
            for i in 0..bucket.len() {
                let v = vat[i];
                let b = bucket[i];
                let mut b1 = (v as f64 / pour_count as f64).ceil() as i32;
                if pour_count == 0 {
                    if v == 0 {
                        continue;
                    } else {
                        b1 = 99999;
                    }
                }
                count += std::cmp::max(b1 - b, 0) as i64;
            }
            //println!("{} {}", count, pour_count);
            result = std::cmp::min(result, count as i32 + pour_count);
        }
        result
    }
}
