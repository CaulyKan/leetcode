use std::collections::HashMap;
impl Solution {
    pub fn min_time(time: Vec<i32>, m: i32) -> i32 {
        fn helper(
            dic: &mut HashMap<(usize, usize, i32), i32>,
            time: &Vec<i32>,
            start: usize,
            end: usize,
            rest: i32,
        ) -> i32 {
            if dic.contains_key(&(start, end, rest)) {
                return dic[&(start, end, rest)];
            }
            if end - start + 1 <= rest as usize {
                return 0;
            }
            let mut result = i32::max_value();
            if rest <= 0 {
                return result;
            }
            for cur in start..(end + 1) {
                let today: Vec<i32> = time
                    .iter()
                    .skip(start)
                    .take(cur - start + 1)
                    .map(|x| *x)
                    .collect();
                let todayavg = today.iter().sum::<i32>() - today.iter().max().unwrap();
                if todayavg > result {
                    continue;
                }
                let restt = helper(dic, &time, cur + 1, end, rest - 1);
                result = std::cmp::min(std::cmp::max(restt, todayavg), result);
            }
            dic.insert((start, end, rest), result);

            result
        }

        let mut dic = HashMap::new();
        let n = time.len();
        if n <= m as usize {
            return 0;
        }
        return helper(&mut dic, &time, 0, n - 1, m);
    }
}

pub struct Solution;
