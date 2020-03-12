/*
 * @lc app=leetcode.cn id=1071 lang=rust
 *
 * [1071] 字符串的最大公因子
 */

// @lc code=start
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        // copy from num::integer::gcd
        /// Calculates the Greatest Common Divisor (GCD) of the number and
        /// `other`. The result is always positive.
        fn gcd(a: usize, b: usize) -> usize {
            let mut m = a;
            let mut n = b;

            // Use Stein's algorithm
            if m == 0 || n == 0 {
                return 0;
            }

            // find common factors of 2
            let shift = (m | n).trailing_zeros();

            // divide n and m by 2 until odd
            // m inside loop
            n >>= n.trailing_zeros();

            while m != 0 {
                m >>= m.trailing_zeros();
                if n > m {
                    ::std::mem::swap(&mut n, &mut m)
                }
                m -= n;
            }

            (n << shift) as usize
        }

        fn check(s: &String, s2: &String) -> bool {
            for i in 0..s2.len() / s.len() {
                if *s != s2[i * s.len()..i * s.len() + s.len()] {
                    return false;
                }
            }
            true
        }

        let gcd_len = gcd(str1.len(), str2.len());

        for f in (1..gcd_len + 1).rev() {
            if gcd_len % f == 0 {
                let s = str1[0..f].to_string();

                if check(&s, &str1) && check(&s, &str2) {
                    return s;
                }
            }
        }

        "".to_string()
    }
}

// @lc code=end
pub struct Solution;
