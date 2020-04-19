impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut vec = vec![1, 2];
        while vec.last().unwrap() < &k {
            let len = vec.len();
            vec.push(vec[len - 1] + vec[len - 2]);
        }

        let mut k = k;
        let mut n = 0;
        for &i in vec.iter().rev() {
            if k >= i {
                k -= i;
                n += 1;
            }
        }

        n
    }
}

pub struct Solution;
