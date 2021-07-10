pub struct Solution;
use cauly_rust_leetcode_utils::graph::*;
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let graph = Graph::new(&times, true, true, |x, y| x);
        let dist = graph.dijkstra_all(k);
        let nodes_dist = graph.nodes().map(|x| dist[x as usize]).collect::<Vec<_>>();
        match nodes_dist.iter().filter(|x| x.is_some()).count() {
            x if x == n as usize => nodes_dist.iter().map(|x| x.unwrap()).max().unwrap(),
            _ => -1,
        }
    }
}
