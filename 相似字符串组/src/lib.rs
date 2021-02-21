pub struct Solution;

use cauly_rust_leetcode_utils::union_find::*;

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut uf = UnionFind4Usize::new(strs.len());
        fn is_similar(str1: &String, str2: &String) -> bool {
            let mut errors = 0;
            for i in 0..str1.len() {
                if str1.as_bytes()[i] != str2.as_bytes()[i] {
                    errors += 1;
                    if errors > 2 {
                        break;
                    }
                }
            }
            //println!("{}, {}, {}", str1, str2, errors);
            errors <= 2
        }
        for i in 0..strs.len() {
            for j in i + 1..strs.len() {
                if is_similar(&strs[i], &strs[j]) {
                    uf.union(i, j);
                    //println!("{}, {}", i, j);
                }
            }
        }

        uf.union_count() as i32
    }
}
