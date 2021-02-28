pub struct Solution;
impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut costs: std::collections::HashSet<i32> = std::collections::HashSet::new();
        for c in base_costs {
            costs.insert(c);
        }

        for i in topping_costs {
            let mut new_costs = Vec::new();
            for c in &costs {
                new_costs.push(c + i);
                new_costs.push(c + 2 * i);
            }
            for c in new_costs {
                costs.insert(c);
            }
        }

        let mut min = 999999;
        let mut result = 999999;
        let mut v = Vec::new();
        for c in costs {
            v.push(c);
        }
        v.sort();
        for c in v.iter().rev() {
            if (target - c).abs() <= min {
                result = *c;
                min = (target - c).abs();
            }
        }

        result
    }
}
