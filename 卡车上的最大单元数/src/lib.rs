pub struct Solution;
impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut boxes = box_types.clone();
        boxes.sort_by(|a, b| b[1].cmp(&a[1]));
        //println!("{:?}", boxes);
        let mut result = 0;
        let mut count = truck_size;
        for i in boxes {
            if count >= i[0] {
                result += i[0] * i[1];
                count -= i[0];
            } else {
                result += count * i[1];
                count = 0;
            }
            if count == 0 {
                break;
            }
        }
        result
    }
}
