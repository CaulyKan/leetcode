pub struct Solution;
use std::collections::HashMap;

impl Solution {
    // pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
    //     let mut bin = vec![0; arr.len()];
    //     fn find_m(m: i32, bin: &Vec<i32>) -> bool {
    //         let mut last = -1;
    //         for i in 0..bin.len() {
    //             if bin[i] == 1 {
    //                 if last == -1 {
    //                     last = 1;
    //                 } else {
    //                     last += 1;
    //                 }
    //             } else {
    //                 if last > 0 {
    //                     if last == m {
    //                         return true;
    //                     }
    //                 }
    //                 last = -1;
    //             }
    //         }
    //         if last > 0 {
    //             if last == m {
    //                 return true;
    //             }
    //         }
    //         false
    //     }

    //     for index in &arr {
    //         bin[(index - 1) as usize] = 1;
    //     }
    //     for round in (1..&arr.len() + 1).rev() {
    //         if find_m(m, &bin) {
    //             return round as i32;
    //         }
    //         bin[(arr[round - 1] - 1) as usize] = 0;
    //     }
    //     -1
    // }
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        let mut bin: Vec<i32> = vec![-1; arr.len()];
        let mut uuid_map: Vec<i32> = vec![];
        let mut uuid = 0;
        let mut last_step = -1;
        let mut step = 0;
        let mut match_count = 0;

        if arr.len() == 1 {
            return if m == 1 { 1 } else { -1 };
        }

        fn check_add(v: &mut i32, add: i32, m: i32, match_count: &mut i32) {
            if *v == m {
                *match_count -= 1;
            }
            *v += add;
            if *v == m {
                *match_count += 1;
            }
        }

        for index in &arr {
            step += 1;
            let i = (*index - 1) as usize;
            if i == 0 {
                if bin[i + 1] != -1 {
                    let id = bin[i + 1];
                    bin[i] = id;
                    check_add(&mut uuid_map[id as usize], 1, m, &mut match_count);
                } else {
                    bin[i] = uuid;
                    uuid_map.push(1);
                    uuid += 1;
                    if 1 == m {
                        match_count += 1;
                    }
                }
            } else if i == arr.len() - 1 {
                if bin[i - 1] != -1 {
                    let id = bin[i - 1];
                    bin[i] = id;
                    check_add(&mut uuid_map[id as usize], 1, m, &mut match_count);
                } else {
                    bin[i] = uuid;
                    uuid_map.push(1);
                    uuid += 1;
                    if 1 == m {
                        match_count += 1;
                    }
                }
            } else {
                if bin[i + 1] != -1 && bin[i - 1] != -1 {
                    let uid_to_add = bin[i - 1];
                    let uid_to_remove = bin[i + 1];
                    let uid_to_remove_len = *(&uuid_map[uid_to_remove as usize]);
                    for j in i..i + 1 + uid_to_remove_len as usize {
                        bin[j] = uid_to_add;
                    }
                    if uid_to_remove_len == m {
                        match_count -= 1;
                    }
                    check_add(
                        &mut uuid_map[uid_to_add as usize],
                        1 + uid_to_remove_len,
                        m,
                        &mut match_count,
                    );
                    uuid_map[uid_to_remove as usize] = -1;
                } else if bin[i - 1] != -1 {
                    let id = bin[i - 1];
                    bin[i] = id;
                    check_add(&mut uuid_map[id as usize], 1, m, &mut match_count);
                } else if bin[i + 1] != -1 {
                    let id = bin[i + 1];
                    bin[i] = id;
                    check_add(&mut uuid_map[id as usize], 1, m, &mut match_count);
                } else {
                    bin[i] = uuid;
                    uuid_map.push(1);
                    uuid += 1;
                    if 1 == m {
                        match_count += 1;
                    }
                }
            }
            if match_count > 0 {
                last_step = step;
            }
            // println!("{:?}", bin);
            // println!("{:?}", uuid_map);
            // println!("-----------")
        }

        last_step
    }
}
