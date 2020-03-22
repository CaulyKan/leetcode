impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        if grid[0].len() == 1 && grid.len() == 1 {
            return true;
        }

        fn next_state(
            grid: &Vec<Vec<i32>>,
            x: i32,
            y: i32,
            dir: i32,
        ) -> Result<(i32, i32, i32), i32> {
            let (newx, newy, dir) = match grid[y as usize][x as usize] {
                1 => match dir {
                    0 => (x + 1, y, 0),
                    2 => (x - 1, y, 2),
                    _ => return Err(-1),
                },
                2 => match dir {
                    1 => (x, y + 1, 1),
                    3 => (x, y - 1, 3),
                    _ => return Err(-1),
                },
                3 => match dir {
                    0 => (x, y + 1, 1),
                    3 => (x - 1, y, 2),
                    _ => return Err(-1),
                },
                4 => match dir {
                    3 => (x + 1, y, 0),
                    2 => (x, y + 1, 1),
                    _ => return Err(-1),
                },
                5 => match dir {
                    0 => (x, y - 1, 3),
                    1 => (x - 1, y, 2),
                    _ => return Err(-1),
                },
                6 => match dir {
                    1 => (x + 1, y, 0),
                    2 => (x, y - 1, 3),
                    _ => return Err(-1),
                },
                _ => return Err(-1),
            };
            if newx < 0 || newx >= grid[0].len() as i32 {
                Err(-1)
            } else if newy < 0 || newy >= grid.len() as i32 {
                Err(-1)
            } else {
                Ok((newx, newy, dir))
            }
        }

        let mut visited = vec![];

        let first = match grid[0][0] {
            1 => (0, 0, 0),
            2 => (0, 0, 1),
            3 => (0, 0, 0),
            4 => (0, 0, 3),
            5 => return false,
            6 => (0, 0, 1),
            _ => return false,
        };

        let mut current = first;
        let mut result = loop {
            visited.push(current);
            match next_state(&grid, current.0, current.1, current.2) {
                Ok(next) => current = next,
                Err(_) => break false,
            }
            println!("{} {} {}", current.0, current.1, current.2);
            if current.0 == (grid[0].len() - 1) as i32 && current.1 == (grid.len() - 1) as i32 {
                break match grid[current.1 as usize][current.0 as usize] {
                    1 => match current.2 {
                        0 => true,
                        2 => true,
                        _ => false,
                    },
                    2 => match current.2 {
                        1 => true,
                        3 => true,
                        _ => false,
                    },
                    3 => match current.2 {
                        0 => true,
                        3 => true,
                        _ => false,
                    },
                    4 => match current.2 {
                        3 => true,
                        2 => true,
                        _ => false,
                    },
                    5 => match current.2 {
                        0 => true,
                        1 => true,
                        _ => false,
                    },
                    6 => match current.2 {
                        1 => true,
                        2 => true,
                        _ => false,
                    },
                    _ => false,
                };
            }
            if visited.contains(&current) {
                break false;
            }
        };

        if !result && grid[0][0] == 4 {
            current = (0, 0, 2);
            result = loop {
                visited.push(current);
                match next_state(&grid, current.0, current.1, current.2) {
                    Ok(next) => current = next,
                    Err(_) => break false,
                }
                if current.0 == (grid[0].len() - 1) as i32 && current.1 == (grid.len() - 1) as i32 {
                    break match grid[current.1 as usize][current.0 as usize] {
                        1 => match current.2 {
                            0 => true,
                            2 => true,
                            _ => false,
                        },
                        2 => match current.2 {
                            1 => true,
                            3 => true,
                            _ => false,
                        },
                        3 => match current.2 {
                            0 => true,
                            3 => true,
                            _ => false,
                        },
                        4 => match current.2 {
                            3 => true,
                            2 => true,
                            _ => false,
                        },
                        5 => match current.2 {
                            0 => true,
                            1 => true,
                            _ => false,
                        },
                        6 => match current.2 {
                            1 => true,
                            2 => true,
                            _ => false,
                        },
                        _ => false,
                    };
                }
                if visited.contains(&current) {
                    break false;
                }
            };
        }

        result
    }
}

pub struct Solution;
