pub struct Solution;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        fn get_factors_sum(num: i32) -> i32 {
            let mut ok = false;
            let mut rr = vec![0, 0];
            for i in 2..((num as f32).sqrt().floor() as i32 + 1) {
                if num % i == 0 {
                    let v = num / i;
                    if v == i {
                        ok = false;
                        break;
                    } else {
                        if ok {
                            ok = false;
                            break;
                        }
                        rr[0] = v;
                        rr[1] = i;
                        ok = true;
                    }
                }
            }
            if ok {
                println!("{} {}", rr[0], rr[1]);
                1 + num + rr[0] + rr[1]
            } else {
                0
            }
        }

        let mut result = 0;
        for num in nums {
            let a = get_factors_sum(num);
            println!("{} {}", num, a);
            result += a;
        }
        result
    }
}
