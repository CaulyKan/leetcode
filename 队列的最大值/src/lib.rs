use std::collections::VecDeque;

pub struct MaxQueue {
    sorted_queue: VecDeque<i32>,
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MaxQueue {
    pub fn new() -> Self {
        MaxQueue {
            sorted_queue: VecDeque::new(),
            queue: VecDeque::new(),
        }
    }
    pub fn max_value(&self) -> i32 {
        match self.sorted_queue.front() {
            Some(i) => *i,
            None => -1,
        }
    }
    pub fn push_back(&mut self, value: i32) {
        while self.sorted_queue.len() > 0 && *self.sorted_queue.back().unwrap() < value {
            self.sorted_queue.pop_back();
        }
        self.sorted_queue.push_back(value);
        self.queue.push_back(value);
    }
    pub fn pop_front(&mut self) -> i32 {
        match self.queue.pop_front() {
            Some(i) => {
                if i == *self.sorted_queue.front().unwrap() {
                    self.sorted_queue.pop_front();
                }
                i
            }
            None => -1,
        }
    }
}
