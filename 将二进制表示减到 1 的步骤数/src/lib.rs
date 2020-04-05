impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut steps = 0;
        let mut nums = vec![];
        let length = s.len();
        let first_len = if s.len() % 64 == 0 { 64 } else { s.len() % 64 };
        for i in 0..(s.len() - 1) / 64 + 1 {
            let n = if i == 0 {
                u64::from_str_radix(&s[0..first_len], 2)
            } else {
                u64::from_str_radix(
                    &s[((i - 1) * 64 + first_len)..std::cmp::min(s.len(), i * 64 + first_len)],
                    2,
                )
            }
            .unwrap();
            nums.push(n);
        }

        loop {
            println!("{:?}", nums);

            if nums.iter().take(nums.len() - 1).all(|&x| x == 0) && nums.last().unwrap() == &1 {
                break;
            } else {
                steps += 1;

                if nums.last().unwrap() % 2 == 0 {
                    let mut o: u128 = 0;
                    for n in nums.iter_mut() {
                        let n128: u128 = o + *n as u128;
                        o = (n128 % 2) * (u64::max_value() as u128 + 1);
                        *n = (n128 / 2) as u64;
                    }
                } else {
                    let mut f = true;
                    for n in nums.iter_mut().rev() {
                        if *n == u64::max_value() && f {
                            *n = 0;
                        } else {
                            *n += 1;
                            f = false;
                            break;
                        }
                    }
                    if f {
                        nums.insert(0, 1);
                    }
                }
            }
        }

        steps
    }
}
pub struct Solution;
