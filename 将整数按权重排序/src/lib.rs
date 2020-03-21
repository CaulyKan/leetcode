pub struct Solution;
impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        fn steps(x: &i32) -> i32 {
            let mut i = *x;
            let mut result = 1;
            while i != 1 {
                if i % 2 == 0 {
                    i = i / 2;
                } else {
                    i = 3 * i + 1
                };
                result += 1;
            }
            result
        }

        let mut vec: Vec<i32> = (lo..(hi + 1)).collect();
        vec.sort_by_cached_key(steps);

        return vec[(k - 1) as usize];
    }
}
