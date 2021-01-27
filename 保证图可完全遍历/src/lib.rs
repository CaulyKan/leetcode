pub struct Solution;

use cauly_rust_leetcode_utils::union_find::UnionFind4Usize;

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind4Usize::new(n as usize);
        let mut result = 0;
        for edge in &edges {
            if edge[0] == 3 {
                if uf.is_connected(edge[1] as usize - 1, edge[2] as usize - 1) {
                    result += 1;
                } else {
                    uf.union(edge[1] as usize - 1, edge[2] as usize - 1);
                }
            }
        }
        let mut uf_alice = uf.clone();
        let mut uf_bob = uf.clone();

        for edge in &edges {
            if edge[0] == 1 {
                if uf_alice.is_connected(edge[1] as usize - 1, edge[2] as usize - 1) {
                    result += 1;
                } else {
                    uf_alice.union(edge[1] as usize - 1, edge[2] as usize - 1);
                }
            } else if edge[0] == 2 {
                if uf_bob.is_connected(edge[1] as usize - 1, edge[2] as usize - 1) {
                    result += 1;
                } else {
                    uf_bob.union(edge[1] as usize - 1, edge[2] as usize - 1);
                }
            }
        }

        if uf_alice.union_count() == 1 && uf_bob.union_count() == 1 {
            result
        } else {
            -1
        }
    }
}
