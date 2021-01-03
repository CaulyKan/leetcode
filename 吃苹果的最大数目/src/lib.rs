pub struct Solution;

use std::cell::RefCell;
use std::cmp::*;
use std::rc::Rc;
#[derive(Eq)]
pub struct Apple {
    create: i32,
    last: i32,
    count: RefCell<i32>,
}
impl Apple {
    pub fn new(create: i32, last: i32, count: i32) -> Self {
        Self {
            create,
            last,
            count: RefCell::new(count),
        }
    }
}
impl Ord for Apple {
    fn cmp(&self, other: &Self) -> Ordering {
        //(self.last + self.create).cmp(&(other.create + other.last))
        (other.last + other.create).cmp(&(self.create + self.last))
    }
}
impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.create == other.create && self.last == other.last
    }
}
impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut pq = std::collections::BinaryHeap::new();
        let mut now = 0;
        let mut eaten = 0;
        for i in 0..apples.len() {
            now = i as i32;
            if apples[i] != 0 {
                pq.push(Apple::new(now, days[i], apples[i]));
            }
            while let Some(a) = pq.peek() {
                if a.create + a.last <= now {
                    pq.pop();
                } else {
                    if *a.count.borrow() == 1 {
                        pq.pop();
                    } else {
                        *a.count.borrow_mut() -= 1;
                    }
                    eaten += 1;
                    break;
                }
            }
        }

        while pq.len() != 0 {
            now += 1;
            while let Some(a) = pq.peek() {
                if a.create + a.last <= now {
                    pq.pop();
                } else {
                    if *a.count.borrow() == 1 {
                        pq.pop();
                    } else {
                        *a.count.borrow_mut() -= 1;
                    }
                    eaten += 1;
                    break;
                }
            }
        }

        eaten
    }
}
