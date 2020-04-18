impl Solution {
    pub fn get_trigger_time(increase: Vec<Vec<i32>>, requirements: Vec<Vec<i32>>) -> Vec<i32> {
        let len = 100001;
        let mut r0 = vec![-1; len];
        let mut r1 = vec![-1; len];
        let mut r2 = vec![-1; len];
        let mut result = vec![-1; requirements.len()];

        r0[0] = 0;
        r1[0] = 0;
        r2[0] = 0;

        let mut current = vec![0, 0, 0];
        for (round, inc) in increase.iter().enumerate() {
            for i in (current[0])..current[0] + inc[0] {
                r0[i as usize + 1] = round as i32 + 1;
            }
            current[0] += inc[0];

            for i in (current[1])..current[1] + inc[1] {
                r1[i as usize + 1] = round as i32 + 1;
            }
            current[1] += inc[1];
            for i in (current[2])..current[2] + inc[2] {
                r2[i as usize + 1] = round as i32 + 1;
            }
            current[2] += inc[2];
        }

        let v: Vec<&i32> = r0.iter().take(10).collect();
        println!("{:?}", v);

        for (pos, req) in requirements.iter().enumerate() {
            let a = r0[req[0] as usize];
            let b = r1[req[1] as usize];
            let c = r2[req[2] as usize];
            if a != -1 && b != -1 && c != -1 {
                result[pos] = std::cmp::max(a, std::cmp::max(b, c));
            }
        }

        result
    }
}

pub struct Solution;
