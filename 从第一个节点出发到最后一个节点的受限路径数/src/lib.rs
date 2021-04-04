pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut map = vec![vec![-1; n as usize]; n as usize];
        for edge in edges {
            map[edge[0] as usize - 1][edge[1] as usize - 1] = edge[2];
            map[edge[1] as usize - 1][edge[0] as usize - 1] = edge[2];
        }
        let mut d2ln = HashMap::new();
        let mut current = HashMap::new();
        current.insert(n - 1, 0 as u64);
        d2ln.insert(n - 1, 0 as u64);
        loop {
            let mut next = HashMap::new();
            for (&index, &weight) in &current {
                for i in 0..n {
                    let weight2 = map[index as usize][i as usize];
                    if weight2 != -1 {
                        let e = d2ln.get(&i);
                        if e.is_none() || *e.unwrap() > weight + weight2 as u64 {
                            *d2ln.entry(i).or_insert(0) = weight + weight2 as u64;
                            *next.entry(i).or_insert(0) = weight + weight2 as u64;
                        }
                    }
                }
            }
            current = next;
            if current.len() == 0 {
                break;
            }
        }

        println!("{:?}", d2ln);

        let mut result = 0;
        let mut current = vec![(n - 1, 0)];
        loop {
            let mut next = Vec::new();
            for &(index, weight) in &current {
                for i in 0..n {
                    if map[index as usize][i as usize] != -1 {
                        let weight2 = d2ln[&i];
                        if weight2 > weight {
                            if i == 0 {
                                result += 1;
                                if result > 1000000000 {
                                    result = result % 1000000009;
                                }
                            } else {
                                next.push((i, weight2));
                            }
                        }
                    }
                }
            }
            current = next;
            //println!("{:?}, {}", current, result);

            if current.len() == 0 {
                break;
            }
        }

        result
    }
}
