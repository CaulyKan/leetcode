pub struct Solution;
impl Solution {
    pub fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut rooms: Vec<(i32, i32)> = rooms.iter().map(|x| (x[0], x[1])).collect();
        rooms.sort_by_cached_key(|x| x.0);
        let mut result = vec![0; queries.len()];

        let mut n = 0;
        for q in queries {
            let qid = q[0];
            let qsize = q[1];
            let mut curid = rooms.last().unwrap().0;
            let mut dist = i32::max_value();
            let mut i = 0;
            let mut vec = Vec::with_capacity(rooms.len());
            for &(id, size) in &rooms {
                if size >= qsize {
                    if (id - qid).abs() < dist {
                        curid = i;
                        dist = (id - qid).abs();
                    }
                    i += 1;
                    vec.push((id, size));
                }
            }
            if vec.len() == 0 {
                result[n] = -1;

                n += 1;
                continue;
            }
            if vec[curid as usize].1 >= qsize {
                result[n] = vec[curid as usize].0;
            } else {
                let mut l = curid;
                let mut r = curid;
                result[n] = loop {
                    if l < 0 && r >= vec.len() as i32 {
                        break -1;
                    } else if r >= vec.len() as i32 {
                        if vec[l as usize].1 >= qsize {
                            break vec[l as usize].0;
                        } else {
                            l -= 1;
                        }
                    } else if l < 0 {
                        if vec[r as usize].1 >= qsize {
                            break vec[r as usize].0;
                        } else {
                            r += 1;
                        }
                    } else if (vec[l as usize].0 - qid).abs() <= (vec[r as usize].0 - qid).abs() {
                        if vec[l as usize].1 >= qsize {
                            break vec[l as usize].0;
                        } else {
                            l -= 1;
                        }
                    } else {
                        if vec[r as usize].1 >= qsize {
                            break vec[r as usize].0;
                        } else {
                            r += 1;
                        }
                    }
                };
            }

            n += 1;
        }
        result
    }
}
