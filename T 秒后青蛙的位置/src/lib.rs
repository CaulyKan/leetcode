use std::collections::HashMap;
impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let n = n as usize;
        fn new_edges(edges: &Vec<Vec<i32>>, exclude: i32) -> Vec<Vec<i32>> {
            let mut result = Vec::new();
            for edge in edges {
                if edge[1] != exclude && edge[0] != exclude {
                    result.push(edge.clone());
                }
            }
            result
        }
        fn next(current: i32, edges: &Vec<Vec<i32>>, t: i32) -> HashMap<i32, f64> {
            let mut result = HashMap::new();
            let mut possible_dest = Vec::new();

            if t == 0 {
                result.insert(current, 1.0);
            } else {
                for edge in edges {
                    if edge[0] == current {
                        possible_dest.push(edge[1]);
                    }
                    if edge[1] == current {
                        possible_dest.push(edge[0]);
                    }
                }
                let l = possible_dest.len();
                if l > 0 {
                    for dest in possible_dest {
                        let p = 1.0 / (l as f64);
                        let all = next(dest, &new_edges(&edges, current), t - 1);
                        for (dest, posi) in all {
                            if result.contains_key(&dest) {
                                let a = result[&dest];
                                result.remove(&dest);
                                result.insert(dest, posi * p + a);
                            } else {
                                result.insert(dest, posi * p);
                            }
                        }
                    }
                } else {
                    result.insert(current, 1.0);
                }
            }

            result
        }

        let all = next(1, &edges, t);
        if all.contains_key(&target) {
            all[&target]
        } else {
            0.0
        }
    }
}

pub struct Solution;
