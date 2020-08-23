impl Solution {
    pub fn minimal_steps(maze: Vec<String>) -> i32 {
        fn route(from: (usize, usize), to: (usize, usize), maze: &Vec<Vec<char>>) -> i32 {
            let rows = maze.len();
            let cols = maze[0].len();
            let mut maze = maze.clone();
            let mut current = vec![from];
            let mut result = 0;
            maze[from.1][from.0] = '#';

            //println!("    {:?}->{:?}", from, to);
            loop {
                let mut next = vec![];
                for pos in &current {
                    if pos.1 == to.1 && pos.0 == to.0 {
                        return result;
                    }
                    if pos.0 > 0 {
                        if maze[pos.1][pos.0 - 1] != '#' {
                            maze[pos.1][pos.0 - 1] = '#';
                            next.push((pos.0 - 1, pos.1));
                        }
                    }
                    if pos.1 > 0 {
                        if maze[pos.1 - 1][pos.0] != '#' {
                            maze[pos.1 - 1][pos.0] = '#';
                            next.push((pos.0, pos.1 - 1));
                        }
                    }
                    if pos.1 < rows - 1 {
                        if maze[pos.1 + 1][pos.0] != '#' {
                            maze[pos.1 + 1][pos.0] = '#';
                            next.push((pos.0, pos.1 + 1));
                        }
                    }
                    if pos.0 < cols - 1 {
                        if maze[pos.1][pos.0 + 1] != '#' {
                            maze[pos.1][pos.0 + 1] = '#';
                            next.push((pos.0 + 1, pos.1));
                        }
                    }
                }
                //println!("    {:?}", next);
                result += 1;
                current = next;
                if current.len() == 0 {
                    return 9999;
                }
            }
        }

        let rows = maze.len();
        let cols = maze[0].len();
        let mut buttons = vec![];
        let mut stones = vec![];
        let mut start = (0, 0);
        let mut target = (0, 0);
        let mut maze: Vec<Vec<char>> = maze.iter().map(|x| x.chars().collect()).collect();
        for i in 0..cols {
            for j in 0..rows {
                if maze[j][i] == 'M' {
                    buttons.push((i, j));
                } else if maze[j][i] == 'O' {
                    stones.push((i, j));
                } else if maze[j][i] == 'S' {
                    start = (i, j);
                } else if maze[j][i] == 'T' {
                    target = (i, j);
                }
            }
        }

        fn total(
            start: (usize, usize),
            target: (usize, usize),
            buttons: &Vec<(usize, usize)>,
            stones: &Vec<(usize, usize)>,
            maze: &Vec<Vec<char>>,
        ) -> i32 {
            let mut current = start;
            let mut result = 0;

            for button in buttons {
                let closest_stone = stones
                    .iter()
                    .min_by_key(|&x| route(current, *x, &maze) + route(*button, *x, &maze))
                    .unwrap();
                let steps_to_stone = route(current, *closest_stone, &maze);
                let stone_to_button = route(*closest_stone, *button, &maze);
                result += steps_to_stone + stone_to_button;
                current = *button;
            }

            result += route(current, target, &maze);

            result
        }
        fn total2(
            start: (usize, usize),
            target: (usize, usize),
            buttons: &Vec<(usize, usize)>,
            stones: &Vec<(usize, usize)>,
            maze: &Vec<Vec<char>>,
        ) -> i32 {
            let mut current = start;
            let mut result = 0;

            for button in buttons {
                let closest_stone = stones
                    .iter()
                    .min_by_key(|&x| route(current, *x, &maze) + route(*button, *x, &maze))
                    .unwrap();
                let steps_to_stone = route(current, *closest_stone, &maze);
                let stone_to_button = route(*closest_stone, *button, &maze);
                println!(
                    "{:?}->{:?}->{:?}, {}, {}",
                    current, *closest_stone, *button, steps_to_stone, stone_to_button
                );
                result += steps_to_stone + stone_to_button;
                current = *button;
            }

            result += route(current, target, &maze);
            println!(
                "{:?}->{:?}, , {}",
                current,
                target,
                route(current, target, &maze)
            );

            result
        }

        let mut act_buttons = vec![];
        for button in buttons {
            if act_buttons.len() == 0 {
                act_buttons.push(button);
            } else {
                let mut min = (i32::max_value(), 0);
                for i in 0..(act_buttons.len() + 1) {
                    let mut btns = act_buttons.clone();
                    if i == act_buttons.len() {
                        btns.push(button);
                    } else {
                        btns.insert(i, button);
                    }
                    let t = total(start, target, &btns, &stones, &maze);
                    if t < min.0 {
                        min = (t, i);
                    }
                }
                if min.1 == act_buttons.len() {
                    act_buttons.push(button);
                } else {
                    act_buttons.insert(min.1, button);
                }
            }
        }
        println!("{:?}", act_buttons);
        let result = total2(start, target, &act_buttons, &stones, &maze);
        if result >= 9999 {
            -1
        } else {
            result
        }
        // if buttons.len() == 1 {
        //     let closest_button = buttons[0];

        //     let closest_stone = stones
        //         .iter()
        //         .min_by_key(|&x| route(closest_button, *x, &maze))
        //         .unwrap();

        //     let steps_to_stone = route(*closest_stone, current, &maze);
        //     let stone_to_button = route(*closest_stone, closest_button, &maze);
        //     if steps_to_stone >= 9999 || steps_to_stone >= 9999 {
        //         return -1;
        //     }
        //     result += steps_to_stone + stone_to_button;
        // } else if buttons.len() > 1 {
        //     let last_button = *(buttons
        //         .iter()
        //         .min_by_key(|&x| route(target, *x, &maze))
        //         .unwrap());
        //     let idx = buttons.iter().position(|x| *x == last_button).unwrap();
        //     buttons.remove(idx);
        //     while buttons.len() > 0 {
        //         let closest_button = *(buttons
        //             .iter()
        //             .min_by_key(|&x| route(current, *x, &maze))
        //             .unwrap());

        //         let closest_stone = stones
        //             .iter()
        //             .min_by_key(|&x| route(current, *x, &maze) + route(closest_button, *x, &maze))
        //             .unwrap();

        //         let steps_to_stone = route(*closest_stone, current, &maze);
        //         let stone_to_button = route(*closest_stone, closest_button, &maze);
        //         if steps_to_stone >= 9999 || steps_to_stone >= 9999 {
        //             return -1;
        //         }
        //         //println!("{:?}", closest_button);

        //         result += steps_to_stone + stone_to_button;

        //         current = closest_button;
        //         let index = buttons.iter().position(|x| *x == closest_button).unwrap();
        //         buttons.remove(index);
        //     }
        //     let closest_stone = *(stones
        //         .iter()
        //         .min_by_key(|&x| route(last_button, *x, &maze))
        //         .unwrap());

        //     //println!("{:?}", last_button);

        //     let steps_to_stone = route(closest_stone, current, &maze);
        //     let stone_to_button = route(closest_stone, last_button, &maze);
        //     if steps_to_stone >= 9999 || steps_to_stone >= 9999 {
        //         return -1;
        //     }
        //     result += steps_to_stone + stone_to_button;
        //     current = last_button;
        // }

        // let steps_to_target = route(current, target, &maze);
        // if steps_to_target >= 9999 {
        //     return -1;
        // }
        // result += steps_to_target;
    }
}

pub struct Solution;
