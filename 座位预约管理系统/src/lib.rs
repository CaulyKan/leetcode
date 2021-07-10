use std::cmp::Reverse;
use std::collections::BinaryHeap;
struct SeatManager {
    heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        let mut h = BinaryHeap::new();
        for i in 1..n + 1 {
            h.push(Reverse(i));
        }
        SeatManager { heap: h }
    }

    fn reserve(&mut self) -> i32 {
        let result = self.heap.pop().unwrap();
        //println!("{}", result);
        result.0
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.heap.push(Reverse(seat_number))
    }
}
