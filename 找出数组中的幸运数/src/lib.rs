impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for i in arr {
            if map.contains_key(&i) {
                *(map.get_mut(&i).unwrap()) += 1;
            } else {
                map.insert(i, 1);
            }
        }

        let mut max = -1;
        for (k, v) in map {
            if k == v {
                max = std::cmp::max(max, v);
            }
        }

        max
    }
}

pub struct Solution;
