pub struct Solution;
use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
        for edge in edges {
            let v = map.entry(edge[0]).or_insert(HashSet::new());
            v.insert(edge[1]);
            let v2 = map.entry(edge[1]).or_insert(HashSet::new());
            v2.insert(edge[0]);
        }
        let mut result = i32::max_value();

        for (from, tos) in &map {
            for to in tos {
                let thirds = map[to].intersection(tos);
                for third in thirds {
                    result = std::cmp::min(
                        result,
                        tos.len() as i32 - 2 + map[to].len() as i32 - 2 + map[third].len() as i32
                            - 2,
                    );
                }
            }
        }

        if result == i32::max_value() {
            -1
        } else {
            result
        }
    }
}
