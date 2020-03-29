use std::collections::HashMap;

struct UndergroundSystem {
    customers: HashMap<i32, (String, i32)>, // key: customer_id, value.0: start station, value.1: start time
    times: HashMap<(String, String), (i32, i32)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        Self {
            customers: HashMap::new(),
            times: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.customers.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (from, s_time) = self.customers.remove(&id).unwrap();
        let entry = self.times.entry((from, station_name)).or_insert((0, 0));
        entry.0 += t - s_time;
        entry.1 += 1;
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        self.times
            .get(&(start_station, end_station))
            .map(|&(s, n)| s as f64 / n as f64)
            .unwrap()
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_underground_system() {
        let mut us = UndergroundSystem::new();
        us.check_in(45, "Leyton".to_string(), 3);
        us.check_in(32, "Paradise".to_string(), 8);
        us.check_in(27, "Leyton".to_string(), 10);
        us.check_out(45, "Waterloo".to_string(), 15);
        us.check_out(27, "Waterloo".to_string(), 20);
        us.check_out(32, "Cambridge".to_string(), 22);
        assert!(
            (us.get_average_time("Paradise".to_string(), "Cambridge".to_string()) - 14.0).abs()
                < 0.00000001
        );
        assert!(
            (us.get_average_time("Leyton".to_string(), "Waterloo".to_string()) - 11.0).abs()
                < 0.00000001
        );
        us.check_in(10, "Leyton".to_string(), 24);
        assert!(
            (us.get_average_time("Leyton".to_string(), "Waterloo".to_string()) - 11.0).abs()
                < 0.00000001
        );
        us.check_out(10, "Waterloo".to_string(), 38);
        assert!(
            (us.get_average_time("Leyton".to_string(), "Waterloo".to_string()) - 12.0).abs()
                < 0.00000001
        );
    }
}
