pub struct Solution;
use cauly_rust_leetcode_utils::define_dp;
use cauly_rust_leetcode_utils::dp::*;
use cauly_rust_leetcode_utils::graph::*;

pub struct State {
    city: usize,
    power: usize,
}

define_dp! {State, city:200, power:100}

impl Solution {
    pub fn electric_car_plan(
        paths: Vec<Vec<i32>>,
        cnt: i32,
        start: i32,
        end: i32,
        charge: Vec<i32>,
    ) -> i32 {
        let n = 100;
        let graph = Graph::new(&paths, true, false, |x, y| std::cmp::min(x, y));
        let mut dp = DP::new::<State>(-1);
        for i in 0..cnt {
            *dp.get_mut(&State {
                city: end as usize,
                power: i as usize,
            }) = 0;
        }

        graph.bfs_rev(vec![end], &mut |source, target| {
            let dist = graph.dist(source, target).unwrap();
            let mut result = false;
            for init_power in 0..cnt + 1 {
                let mut total_time = i32::max_value();
                for remain_power in 0..cnt - dist + 1 {
                    let charge_power = std::cmp::max(remain_power + dist - init_power, 0);
                    let charge_time = charge_power * charge[source as usize];
                    let t = charge_time
                        + dist
                        + dp.get(&State {
                            city: target as usize,
                            power: remain_power as usize,
                        });
                    total_time = std::cmp::min(total_time, t);
                }
                let v = dp.get(&State {
                    city: source as usize,
                    power: init_power as usize,
                });
                if v == -1 || v > total_time {
                    *dp.get_mut(&State {
                        city: source as usize,
                        power: init_power as usize,
                    }) = total_time;
                    result = true;
                }
            }
            result
        });

        dp.get(&State {
            city: start as usize,
            power: 0,
        })
    }
}
