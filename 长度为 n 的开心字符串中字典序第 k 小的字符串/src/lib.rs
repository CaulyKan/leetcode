impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let max = i32::pow(3, n as u32);
        fn is_happy(num: i32, n: i32) -> bool {
            let mut last = -1;
            let mut current = num;
            for i in 0..n {
                if last == current % 3 {
                    return true;
                }
                last = current % 3;
                current = current / 3;
            }
            false
        }

        let mut i = 0;
        let mut current = -1;
        while i < k {
            current += 1;
            if !is_happy(current, n) {
                i += 1;
            }
            if current >= max {
                return "".to_string();
            }
            //println!("{}: {}", tostr(current, n), is_happy(current, n));
        }

        fn tostr(current: i32, n: i32) -> String {
            let mut current = current;
            let mut result = String::with_capacity(10);
            let mut v = vec![];
            while current != 0 {
                v.push(current % 3);
                current /= 3;
            }
            while v.len() < n as usize {
                v.push(0);
            }
            for &n in v.iter().rev() {
                if n == 0 {
                    result.push('a');
                } else if n == 1 {
                    result.push('b');
                } else if n == 2 {
                    result.push('c');
                }
            }
            result
        }
        tostr(current, n)
    }
}

pub struct Solution;
