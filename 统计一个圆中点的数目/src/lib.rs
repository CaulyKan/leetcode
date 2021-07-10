pub struct Solution;
impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0; queries.len()];

        let mut i = 0;
        for q in queries {
            let (qx, qy, qr) = (q[0] as i64, q[1] as i64, q[2] as i64);
            for p in points.iter() {
                let (px, py) = (p[0] as i64, p[1] as i64);
                if (px - qx).pow(2) + (py - qy).pow(2) <= qr.pow(2) {
                    result[i] += 1;
                }
            }
            i += 1;
        }

        result
    }
}
