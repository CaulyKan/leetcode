pub struct Solution;
impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut result = vec![0; k as usize];
        let mut map: std::collections::HashMap<i32, std::collections::HashSet<i32>> =
            std::collections::HashMap::new();
        for log in logs {
            if !map.contains_key(&log[0]) {
                map.insert(log[0], std::collections::HashSet::new());
            }
            (map.get_mut(&log[0]).unwrap()).insert(log[1]);
        }

        for (_, set) in map {
            result[set.len()-1] += 1;
        }

        result
    }
}
