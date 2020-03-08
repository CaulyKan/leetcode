impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut time = vec![-1; n];
        let mut known_count = 1;
        time[head_id as usize] = 0;
        loop {
            for person in 0..n {
                if time[person] == -1 {
                    let manager_time = time[manager[person] as usize];
                    if manager_time != -1 {
                        time[person] = manager_time + inform_time[manager[person] as usize];
                        known_count += 1;
                    }
                }
            }
            if known_count == n {
                break;
            }
        }
        return *time.iter().max().unwrap();
    }
}

pub struct Solution;
