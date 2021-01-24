pub struct Solution;
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let product = nums[i] * nums[j];
                let counter = map.entry(product).or_insert(0);
                *counter += 1;
            }
        }

        let mut result = 0;
        for i in map.values() {
            if *i > 1 {
                result += (*i - 1) * (*i) / 2 * 8;
            }
        }

        //        println!("{:?}", map);

        result
    }
}
