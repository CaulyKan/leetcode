impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        for i in 1..(n + 1) {
            let mut sum = 0;
            let mut c = i;
            loop {
                sum += c % 10;
                if c < 10 {
                    break;
                } else {
                    c = c / 10;
                }
            }
            if !map.contains_key(&sum) {
                map.insert(sum, vec![i]);
            } else {
                map.get_mut(&sum).unwrap().push(i);
            }
        }

        let mut max = 0;
        let mut count = 0;
        for (_, v) in map {
            println!("{:?}", v);
            if v.len() > max {
                max = v.len();
                count = 1;
            } else if v.len() == max {
                count += 1;
            }
        }

        count
    }
}

pub struct Solution;
