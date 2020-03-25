/*
 * @lc app=leetcode.cn id=892 lang=rust
 *
 * [892] 三维形体的表面积
 */

// @lc code=start
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let row = grid.len();
        let col = grid[0].len();
        let mut count = 0;
        let mut near = 0;
        for y in 0..row {
            for x in 0..col {
                if x > 0 {
                    near += std::cmp::min(grid[y][x - 1], grid[y][x]);
                }

                if y > 0 {
                    near += std::cmp::min(grid[y - 1][x], grid[y][x]);
                }

                if grid[y][x] > 0 {
                    near += grid[y][x] - 1;
                }
                count += grid[y][x];
            }
        }
        count * 6 - near * 2
    }
}
// @lc code=end
pub struct Solution;
