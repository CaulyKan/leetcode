use std::cell::RefCell;
use std::collections::HashMap;
struct UndergroundSystem {
    map: RefCell<HashMap<(String, String), Vec<i32>>>,
    inprogress: RefCell<HashMap<i32, (String, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        UndergroundSystem {
            map: RefCell::new(HashMap::new()),
            inprogress: RefCell::new(HashMap::new()),
        }
    }
    fn check_in(&self, id: i32, station_name: String, t: i32) {
        let mut inp = self.inprogress.borrow_mut();
        inp.insert(id, (station_name, t));
    }

    fn check_out(&self, id: i32, station_name: String, t: i32) {
        let mut inp = self.inprogress.borrow_mut();
        let mut m = self.map.borrow_mut();
        let (in_station, tin) = inp.get(&id).unwrap();

        let key = (in_station.clone(), station_name);

        if m.contains_key(&key) {
            let c = m.get_mut(&key).unwrap();
            c.push(t - tin);
        } else {
            m.insert(key, vec![t - tin]);
        }
        inp.remove(&id);
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let m = self.map.borrow();
        let vec = m.get(&(start_station, end_station)).unwrap();
        let mut result = 0.0;

        for i in vec {
            result += *i as f64;
        }

        result / (vec.len() as f64)
    }
}
