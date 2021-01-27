pub struct Solution;

use cauly_rust_leetcode_utils::grid::*;
use cauly_rust_leetcode_utils::union_find::*;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let mut vec = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                vec.push((j, i, grid[i][j]));
            }
        }
        println!("{:?}", vec);
        let g = Grid::from(grid.clone());

        let mut uf = UnionFind::from_iter(vec.clone());

        vec.sort_by(|x, y| x.2.cmp(&y.2));

        let start = (0, 0, grid[0][0]);
        let finish = (
            grid[0].len() - 1,
            grid.len() - 1,
            grid[grid[0].len() - 1][grid.len() - 1],
        );

        let mut cur = 0;
        for t in 0..50 * 50 {
            for i in cur..vec.len() {
                let item = vec[i];
                if item.2 < t {
                    for neibour in g.get_near_4((item.0, item.1)) {
                        if neibour.val < t {
                            uf.union(item, (neibour.x, neibour.y, neibour.val)).unwrap();
                        }
                    }
                    cur = i;
                }
            }
            if uf.is_connected(start, finish).unwrap() {
                return t - 1;
            }
        }

        0
    }
}
