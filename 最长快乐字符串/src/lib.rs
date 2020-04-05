use std::cmp::max;
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut a = a;
        let mut b = b;
        let mut c = c;
        let mut ad = (a + 1) / 2;
        let mut bd = (b + 1) / 2;
        let mut cd = (c + 1) / 2;
        let mut result = String::new();
        let mut last = 0;

        fn checka(last: i32, ad: i32, bd: i32, cd: i32) -> bool {
            if last == 3 {
                ad + bd > cd
            } else {
                ad + cd > bd
            }
        }

        fn checkb(last: i32, ad: i32, bd: i32, cd: i32) -> bool {
            if last == 3 {
                ad + bd > cd
            } else {
                bd + cd > ad
            }
        }

        fn checkc(last: i32, ad: i32, bd: i32, cd: i32) -> bool {
            if last == 1 {
                cd + bd > ad
            } else {
                ad + cd > bd
            }
        }

        loop {
            println!("{}", result);
            if ad >= if last == 2 { -1 } else { bd }
                && ad >= if last == 3 { -1 } else { cd }
                && last != 1
            {
                if a > 1 && checka(last, ad, bd, cd) {
                    a -= 2;
                    result.push_str("aa");
                } else {
                    a -= 1;
                    result.push_str("a");
                }
                ad = (a + 1) / 2;
                last = 1;
            } else if bd >= if last == 1 { -1 } else { ad }
                && bd >= if last == 3 { -1 } else { cd }
                && last != 2
            {
                if b > 1 && checkb(last, ad, bd, cd) {
                    b -= 2;
                    result.push_str("bb");
                } else {
                    b -= 1;
                    result.push_str("b");
                }
                bd = (b + 1) / 2;
                last = 2;
            } else if cd >= if last == 2 { -1 } else { bd }
                && cd >= if last == 1 { -1 } else { ad }
                && last != 3
            {
                if c > 1 && checkc(last, ad, bd, cd) {
                    c -= 2;
                    result.push_str("cc");
                } else {
                    c -= 1;
                    result.push_str("c");
                }
                cd = (c + 1) / 2;
                last = 3;
            }
            if a == 0 && b == 0 || a == 0 && c == 0 || b == 0 && c == 0 {
                if a > 1 {
                    result.push_str("aa");
                } else if a > 0 {
                    result.push_str("a");
                }
                if b > 1 {
                    result.push_str("bb");
                } else if b > 0 {
                    result.push_str("b");
                }
                if c > 1 {
                    result.push_str("cc");
                } else if c > 0 {
                    result.push_str("c");
                }
                break;
            }
        }
        result
    }
}

pub struct Solution;
