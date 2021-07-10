pub struct Solution;
fn GCD(m: i32, n: i32) -> i32 {
    let mut m = m;
    let mut n = n;
    let mut t = 0;
    let mut r = 0;
    if m < n {
        t = m;
        m = n;
        n = t;
    }
    while (m % n) != 0 {
        r = m % n;
        m = n;
        n = r;
    }
    n
}

impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        let mut max1 = -1;
        let mut result = 0;
        for i in nums {
            max1 = if i > max1 { i } else { max1 };
            set.insert(i);
        }

        for n in 1..max1 + 1 {
            'a: for &i in &set {
                if i == n {
                    print!("{}", n);
                    result += 1;
                    break;
                }
                for &j in &set {
                    if i != j {
                        if GCD(i, j) != n {
                            print!("{}", n);
                            result += 1;
                            break 'a;
                        }
                    }
                }
            }
        }

        result
    }
}
