pub struct Solution;
impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let mut target = vec![];
        for i in 0..22 {
            target.push(2i32.pow(i));
        }

        fn ispair(n: i32) -> bool {
            if n == 0 {
                return false;
            }
            n & (n - 1) == 0
        }

        let mut map = std::collections::HashMap::new();
        for i in deliciousness {
            if map.contains_key(&i) {
                *map.get_mut(&i).unwrap() += 1;
            } else {
                map.insert(i, 1);
            }
        }

        let mut result: i64 = 0;
        let keys: Vec<&i32> = map.keys().collect();
        for ii in 0..keys.len() {
            let i = *keys[ii];
            if map[&i] > 1 && ispair(i * 2) {
                result += ((map[&i] - 1) * map[&i] / 2);
            }
            for j in &target {
                if i < *j - i {
                    if let Some(jj) = map.get(&(*j - i)) {
                        println!("{} {}", i, j - i);
                        result += (map[&i] * jj);
                    }
                }
            }
            if result > 1000000007 {
                result = result % (1000000007);
            }
        }
        result as i32
    }

    pub fn count_pairs2(deliciousness: Vec<i32>) -> i32 {
        fn ispair(n: i32) -> bool {
            if n == 0 {
                return false;
            }
            n & (n - 1) == 0
        }

        let mut map = std::collections::HashMap::new();
        for i in deliciousness {
            if map.contains_key(&i) {
                *map.get_mut(&i).unwrap() += 1;
            } else {
                map.insert(i, 1);
            }
        }

        //println!("{:?}", map);

        let mut result = 0;
        let keys: Vec<&i32> = map.keys().collect();
        for ii in 0..keys.len() {
            for jj in ii..keys.len() {
                let i = keys[ii];
                let j = keys[jj];
                if *i == *j {
                    if map[i] > 1 && ispair(*i * 2) {
                        result += (map[i] - 1) * map[i] / 2;
                    }
                } else {
                    if ispair(*i + *j) {
                        result += map[i] * map[j];
                    }
                }
                if result > 1000000007 {
                    result = result % 1000000007;
                }
            }
        }

        result
    }
}
