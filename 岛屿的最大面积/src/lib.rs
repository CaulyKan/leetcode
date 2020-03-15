/*
 * @lc app=leetcode.cn id=695 lang=rust
 *
 * [695] 岛屿的最大面积
 */

// @lc code=start
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let row = grid.len();
        let col = grid[0].len();
        let mut grid = grid.clone();
        let mut queue = std::collections::VecDeque::new();
        let mut max = 0;

        fn get_neibours(
            p: (usize, usize),
            row: usize,
            col: usize,
            queue: &mut std::collections::VecDeque<(usize, usize)>,
        ) {
            if p.0 > 0 {
                queue.push_back((p.0 - 1, p.1));
            }
            if p.0 < row - 1 {
                queue.push_back((p.0 + 1, p.1));
            }
            if p.1 > 0 {
                queue.push_back((p.0, p.1 - 1));
            }
            if p.1 < col - 1 {
                queue.push_back((p.0, p.1 + 1));
            }
        }

        for r in 0..row {
            for c in 0..col {
                if grid[r][c] == 1 {
                    let mut area = 1;
                    grid[r][c] = 0;
                    get_neibours((r, c), row, col, &mut queue);
                    while queue.len() > 0 {
                        let current = queue.pop_front().unwrap();
                        if grid[current.0][current.1] == 1 {
                            area += 1;
                            get_neibours(current, row, col, &mut queue);
                            grid[current.0][current.1] = 0;
                        }
                    }

                    max = std::cmp::max(max, area);
                }
            }
        }

        max
    }
}
// @lc code=end

pub struct Solution;
