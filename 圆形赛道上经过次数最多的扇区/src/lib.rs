pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct Pair {
    key: i32,
    value: i32,
}

impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let mut counter = vec![];
        for i in 1..n + 1 {
            counter.push(Pair { key: i, value: 0 });
        }
        counter[(rounds[0] - 1) as usize].value = 1;
        fn calc_round(n: i32, start: i32, stop: i32, counter: &mut std::vec::Vec<Pair>) {
            println!("start: {}, stop: {}", start, stop);
            let mut i = start;
            loop {
                i += 1;
                if i > n {
                    i = 1;
                }
                counter[(i - 1) as usize].value += 1;
                if i == stop {
                    break;
                }
            }
        }

        for round in 0..rounds.len() - 1 {
            calc_round(n, rounds[round], rounds[round + 1], &mut counter);
        }

        counter.sort_by_cached_key(|x| -x.value);

        println!("{:?}", counter);

        let mut result = vec![];
        let max = counter[0].value;
        for i in counter {
            if i.value == max {
                result.push(i.key);
            }
        }

        result
    }
}
