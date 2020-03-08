impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let is_odd = n % 2 != 0;
        if is_odd {
            std::iter::repeat("a").take(n as usize).collect::<String>()
        } else {
            std::iter::repeat("a")
                .take((n - 1) as usize)
                .collect::<String>()
                + "b"
        }
    }
}

pub struct Solution;
