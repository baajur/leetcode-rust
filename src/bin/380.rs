use rand::prelude::*;
use std::collections::HashMap;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
    rng: ThreadRng,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            list: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            false
        } else {
            self.list.push(val);
            self.map.insert(val, self.list.len() - 1);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            let index = self.map.remove(&val).unwrap();
            let last_index = self.list.len() - 1;
            if last_index >= 1 && last_index != index {
                self.list.swap(index, last_index);
                self.list.pop();
                self.map.entry(self.list[index]).and_modify(|x| *x = index);
            } else {
                self.list.pop();
            }
            true
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        let random = self.rng.gen_range(0, self.list.len());
        self.list[random]
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randomized_set() {
        let mut rset = RandomizedSet::new();
        assert!(rset.insert(1));
        assert!(!rset.insert(1));
        assert!(!rset.remove(2));
        assert!(rset.insert(2));
        let val = rset.get_random();
        assert!(val == 1 || val == 2);
        let val = rset.get_random();
        assert!(val == 1 || val == 2);
        let val = rset.get_random();
        assert!(val == 1 || val == 2);
        assert!(rset.remove(1));
        assert_eq!(rset.get_random(), 2);
        assert_eq!(rset.get_random(), 2);
        assert_eq!(rset.get_random(), 2);
        assert_eq!(rset.get_random(), 2);

        let mut rset = RandomizedSet::new();
        assert!(!rset.remove(0));
        assert!(!rset.remove(0));
        assert!(rset.insert(0));
        assert_eq!(rset.get_random(), 0);
        assert_eq!(rset.get_random(), 0);
        assert_eq!(rset.get_random(), 0);
        assert!(rset.remove(0));
        assert!(rset.insert(0));

        let mut rset = RandomizedSet::new();
        assert!(rset.insert(3));
        assert!(!rset.insert(3));
        assert_eq!(rset.get_random(), 3);
        assert_eq!(rset.get_random(), 3);
        assert_eq!(rset.get_random(), 3);
        assert!(rset.insert(1));
        assert!(rset.remove(3));
        assert_eq!(rset.get_random(), 1);
        assert_eq!(rset.get_random(), 1);
        assert_eq!(rset.get_random(), 1);
        assert!(rset.insert(0));
        assert!(rset.remove(0));
    }
}
