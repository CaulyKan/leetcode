pub struct Solution;
impl Solution {
    pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        let mut result = vec![-1f64; cars.len()];

        fn chase(car1pos: i32, car1spd: i32, car2pos: i32, car2spd: i32) -> f64 {
            if car1spd <= car2spd {
                -1.0
            } else {
                (car2pos as f64 - car1pos as f64) / (car1spd as f64 - car2spd as f64)
            }
        }

        let mut curslowest = cars[cars.len() - 1][1];
        for i in (0..cars.len() - 1).rev() {
            if cars[i][1] <= curslowest {
                result[i] = -1.0;
            } else {
                for j in i + 1..cars.len() {
                    let time2chasej = chase(cars[i][0], cars[i][1], cars[j][0], cars[j][1]);
                    //println!("{} time to chase {}: {}", i, j, time2chasej);
                    if time2chasej == -1.0 || (result[j] > 0.0 && time2chasej > result[j]) {
                        if j == cars.len() - 1 {
                            result[i] = -1.0;
                        }
                    } else {
                        result[i] = time2chasej;
                        break;
                    }
                }
            }
            curslowest = std::cmp::min(curslowest, cars[i][1]);
        }

        result
    }
}
