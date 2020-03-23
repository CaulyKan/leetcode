pub struct CustomStack {
    max_size: usize,
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    pub fn new(maxSize: i32) -> Self {
        CustomStack {
            max_size: maxSize as usize,
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size {
            self.stack.push(x);
        }
    }

    pub fn pop(&mut self) -> i32 {
        match self.stack.pop() {
            Some(x) => x,
            None => -1,
        }
    }

    pub fn increment(&mut self, k: i32, val: i32) {
        for i in 0..(std::cmp::min(k as usize, self.stack.len())) {
            self.stack[i] += val;
        }
    }
}
