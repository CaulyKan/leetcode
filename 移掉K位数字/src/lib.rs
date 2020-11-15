impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut result = vec![];
        let mut r: Vec<char> = num.chars().collect();
        let mut current = k;

        if k == 0 {
            return num;
        }

        fn helper(l: &mut Vec<char>, r: &mut Vec<char>, k: &mut i32) {
            let mut min_index = 0;
            let mut min = r[0];
            for i in 1..(*k + 1) as usize {
                if r[i] < min {
                    min = r[i];
                    min_index = i;
                }
            }
            if min_index == 0 {
                if !(l.len() == 0 && r[0] == '0') {
                    l.push(r[0]);
                }
                r.drain(0..1);
            } else {
                *k -= min_index as i32;
                r.drain(0..min_index);
            }
            //println!("{:?} {:?} {}", l, r, k);
        }

        while k > 0 && r.len() > current as usize {
            helper(&mut result, &mut r, &mut current);
        }

        if result.len() == 0 {
            return String::from("0");
        }

        result.iter().collect()
    }
}
pub struct Solution;
