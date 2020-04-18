use std::collections::HashMap;
impl Solution {
    pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut dic: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in relation {
            let key = i[0];
            let val = i[1];
            if dic.contains_key(&key) {
                dic.get_mut(&key).unwrap().push(val);
            } else {
                dic.insert(key, vec![val]);
            }
        }

        let mut result = 0;
        let mut current_list = vec![0];
        for _ in 0..k {
            let mut next_list = vec![];
            for current in &current_list {
                if let Some(list) = dic.get(&current) {
                    for &next in list {
                        next_list.push(next);
                    }
                }
            }
            current_list = next_list;
        }

        for item in current_list {
            if item == n - 1 {
                result += 1;
            }
        }

        result
    }
}

pub struct Solution;
