impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut p: Vec<i32> = (1..m + 1).collect();
        for i in 0..queries.len() {
            let q = queries[i];
            let pos = p.iter().position(|&x| x == q).unwrap();
            let val = p[pos];
            for n in (0..pos).rev() {
                p[n + 1] = p[n];
            }
            p[0] = val;
            result.push(pos as i32);
        }
        result
    }
}

pub struct Solution;
