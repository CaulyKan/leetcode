impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let fr = radius as f64;
        fn distence(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
            return (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f64).sqrt();
        }

        if (x1 - x_center).abs() <= radius {
            if y1 <= y_center && y2 >= y_center || y1 >= y_center && y2 <= y_center {
                return true;
            } else if distence(x1, y1, x_center, y_center) < fr
                || distence(x1, y2, x_center, y_center) < fr
            {
                return true;
            }
        }

        if (x2 - x_center).abs() <= radius {
            if y1 <= y_center && y2 >= y_center || y1 >= y_center && y2 <= y_center {
                return true;
            } else if distence(x2, y1, x_center, y_center) < fr
                || distence(x2, y2, x_center, y_center) < fr
            {
                return true;
            }
        }

        if (y2 - y_center).abs() <= radius {
            if x1 <= x_center && x2 >= x_center || x1 >= x_center && x2 <= x_center {
                return true;
            } else if distence(x1, y2, x_center, y_center) < fr
                || distence(x2, y2, x_center, y_center) < fr
            {
                return true;
            }
        }

        if (y1 - y_center).abs() <= radius {
            if x1 <= x_center && x2 >= x_center || x1 >= x_center && x2 <= x_center {
                return true;
            } else if distence(x1, y1, x_center, y_center) < fr
                || distence(x2, y1, x_center, y_center) < fr
            {
                return true;
            }
        }

        if (x1 >= x_center && x2 <= x_center || x1 <= x_center && x2 >= x_center)
            && (y1 >= y_center && y2 <= y_center || y1 <= y_center && y2 >= y_center)
        {
            return true;
        }

        false
    }
}

pub struct Solution;
