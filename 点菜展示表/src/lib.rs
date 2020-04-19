use std::collections::*;
impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut dic: BTreeMap<i32, HashMap<String, i32>> = BTreeMap::new();
        let mut food = BTreeSet::new();

        for order in orders {
            let f = order[2].clone();
            let table = order[1].parse::<i32>().unwrap();
            food.insert(f.clone());
            if let Some(d) = dic.get_mut(&table) {
                if let Some(v) = d.get_mut(&f) {
                    *v += 1;
                } else {
                    d.insert(f.clone(), 1);
                }
            } else {
                let mut m = HashMap::new();
                m.insert(f.clone(), 1);
                dic.insert(table, m);
            }
        }

        let mut result = vec![vec!["".to_string(); food.len() + 1]; dic.len() + 1];
        result[0][0] = "Table".to_string();
        for (i, f) in food.iter().enumerate() {
            result[0][i + 1] = f.clone();
            for (j, (t, order)) in dic.iter().enumerate() {
                result[j + 1][0] = t.to_string();
                result[j + 1][i + 1] = order.get(f).unwrap_or(&0).to_string();
            }
        }
        result
    }
}

pub struct Solution;
