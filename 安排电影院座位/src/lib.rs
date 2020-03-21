impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        // let mut result = vec![[0; 4]; n as usize];
        // for reserved in reserved_seats {
        //     if reserved[1] < 4 && reserved[1] > 1 {
        //         result[(reserved[0] - 1) as usize][0] = 1;
        //     } else if reserved[1] >= 4 && reserved[1] <= 5 {
        //         result[(reserved[0] - 1) as usize][1] = 1;
        //     } else if reserved[1] >= 6 && reserved[1] <= 7 {
        //         result[(reserved[0] - 1) as usize][2] = 1;
        //     } else if reserved[1] >= 8 && reserved[1] <= 9 {
        //         result[(reserved[0] - 1) as usize][3] = 1;
        //     }
        // }
        // let mut count = 0 as i32;
        // for row in result {
        //     if row[0] == 0 && row[1] == 0 && row[2] == 0 && row[3] == 0 {
        //         count += 2;
        //     } else if row[0] == 0 && row[1] == 0 {
        //         count += 1;
        //     } else if row[2] == 0 && row[3] == 0 {
        //         count += 1;
        //     } else if row[1] == 0 && row[2] == 0 {
        //         count += 1;
        //     }
        // }
        // count

        let mut seats = reserved_seats.clone();
        seats.sort_by_cached_key(|x| x[0]);
        println!("{:?}", seats);

        let mut count = 0;
        let mut row = [0; 4];
        let mut current_row = 1;
        for s in seats {
            if s[0] > current_row {
                if row[0] == 0 && row[1] == 0 && row[2] == 0 && row[3] == 0 {
                    count += 2;
                } else if row[0] == 0 && row[1] == 0 {
                    count += 1;
                } else if row[2] == 0 && row[3] == 0 {
                    count += 1;
                } else if row[1] == 0 && row[2] == 0 {
                    count += 1;
                }

                count += (s[0] - current_row - 1) * 2;

                row[0] = 0;
                row[1] = 0;
                row[2] = 0;
                row[3] = 0;

                current_row = s[0];
            }

            if s[1] < 4 && s[1] > 1 {
                row[0] = 1;
            } else if s[1] >= 4 && s[1] <= 5 {
                row[1] = 1;
            } else if s[1] >= 6 && s[1] <= 7 {
                row[2] = 1;
            } else if s[1] >= 8 && s[1] <= 9 {
                row[3] = 1;
            }
        }

        if row[0] == 0 && row[1] == 0 && row[2] == 0 && row[3] == 0 {
            count += 2;
        } else if row[0] == 0 && row[1] == 0 {
            count += 1;
        } else if row[2] == 0 && row[3] == 0 {
            count += 1;
        } else if row[1] == 0 && row[2] == 0 {
            count += 1;
        }

        count += (n - current_row) * 2;

        count
    }
}

pub struct Solution;
