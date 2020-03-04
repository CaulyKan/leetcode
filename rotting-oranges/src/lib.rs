pub struct Solution;
impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let w = grid[0].len();
        let h = grid.len();
        let mut bad = Vec::new();
        let mut good = Vec::new();
        for x in 0..w {
            for y in 0..h {
                if grid[y][x] == 1 {
                    good.push((x, y));
                } else if grid[y][x] == 2 {
                    bad.push((x, y));
                }
            }
        }
        let mut num = good.len();
        if num == 0 {
            return 0;
        }

        let mut days = 0;
        while num > 0 {
            let mut newbad = Vec::new();
            for &(x, y) in bad.iter() {
                let mut neibours = Vec::new();
                if x > 0 {
                    neibours.push((x - 1, y))
                }
                if y > 0 {
                    neibours.push((x, y - 1));
                }
                if x < w - 1 {
                    neibours.push((x + 1, y));
                }
                if y < h - 1 {
                    neibours.push((x, y + 1))
                }

                for &(x2, y2) in neibours.iter() {
                    let good_index = good.iter().position(|&(ax, ay)| ax == x2 && ay == y2);
                    if good_index.is_some() {
                        newbad.push((x2, y2));
                        good.swap_remove(good_index.unwrap());
                    }
                }
            }
            bad = newbad;
            if num == good.len() {
                return -1;
            }
            num = good.len();
            days += 1;
        }
        days
    }
}
