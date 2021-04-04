pub struct Solution;
use std::cmp::*;
pub struct Class {
    pass_rate: f64,
    total: i32,
    more:f64
}
impl Ord for Class {
    fn cmp(&self, other: &Self) -> Ordering {
        //(self.last + self.create).cmp(&(other.create + other.last))
        ((other.more*100000.0) as i64).cmp(&((self.more*100000.0) as i64))
    }
}
impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.create == other.create && self.last == other.last
    }
}
impl<'a> Eq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.create == other.create && self.last == other.last
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut pq = std::collections::BinaryHeap();
        let n = classes.len();
        for i in 0..n{
            let 
        }
    }
}
