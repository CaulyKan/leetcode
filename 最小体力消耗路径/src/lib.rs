pub struct Solution;

use cauly_rust_leetcode_utils::grid::*;
use std::collections::*;
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut grid = Grid::from(heights);
        if grid.rows() == 1 && grid.cols() == 1 {
            return 0;
        }
        let mut result = 0;
        let mut set = HashMap::new();
        for neibour in grid.get_near_4((0, 0)) {
            let v = neibour.val;
            grid.set_is_disabled(neibour.get_pos(), true);
            set.insert(neibour, (v - grid.get((0, 0)).val).abs());
        }
        grid.set_is_disabled((0, 0), true);
        loop {
            let mut temp = HashMap::new();
            let mut temp2 = HashSet::new();
            for (cur, &val) in &set {
                if val <= result {
                    if cur.y == grid.rows() - 1 && cur.x == grid.cols() - 1 {
                        return result;
                    }
                    grid.set_is_disabled(cur.get_pos(), false);
                    for neibour in grid.get_near_4(cur.get_pos()) {
                        let v = neibour.val;
                        let v2 = set.get(&neibour).unwrap_or(&i32::max_value());
                        let p = temp.entry(neibour).or_insert(i32::max_value());
                        *p = std::cmp::min(*v2, std::cmp::min(*p, (v - cur.val).abs()));
                    }
                    temp2.insert((*cur).clone());
                }
            }

            //break 0;
            println!("{}: {:?}", result, set);
            if temp.len() == 0 {
                result += 1;
            } else {
                for (k, v) in temp {
                    set.insert(k, v);
                }

                for item in temp2 {
                    set.remove(&item);
                }
            }
            //println!("{}: {:?}", result, set);
        }
    }
}
