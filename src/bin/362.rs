struct HitCounter {
    hits: Vec<(i32, i32)>,
}

impl HitCounter {
    fn new() -> Self {
        HitCounter { hits: Vec::new() }
    }

    fn hit(&mut self, timestamp: i32) {
        if self.hits.is_empty() {
            self.hits.push((timestamp, 1));
        } else {
            let last = self.hits.last_mut().unwrap();
            if last.0 == timestamp {
                last.1 += 1;
            } else {
                self.hits.push((timestamp, 1));
            }
        }
    }

    fn get_hits(&self, timestamp: i32) -> i32 {
        let lower_bound = {
            let mut ok = self.hits.len() as i32;
            let mut ng = -1;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                if self.hits[mid as usize].0 >= timestamp - 299 {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        };
        let upper_bound = {
            let mut ok = self.hits.len() as i32;
            let mut ng = -1;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                if self.hits[mid as usize].0 > timestamp {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        };

        let mut cnt = 0;
        for i in lower_bound..upper_bound {
            cnt += self.hits[i as usize].1;
        }
        cnt
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_counter() {
        let mut c = HitCounter::new();
        c.hit(1);
        assert_eq!(c.get_hits(4), 1);
        c.hit(2);
        assert_eq!(c.get_hits(4), 2);
        c.hit(3);
        assert_eq!(c.get_hits(3), 3);
        assert_eq!(c.get_hits(4), 3);
        c.hit(300);
        assert_eq!(c.get_hits(300), 4);
        assert_eq!(c.get_hits(301), 3);
        assert_eq!(c.get_hits(302), 2);
        assert_eq!(c.get_hits(303), 1);
        assert_eq!(c.get_hits(304), 1);
        assert_eq!(c.get_hits(500), 1);
        assert_eq!(c.get_hits(599), 1);
        assert_eq!(c.get_hits(600), 0);
    }
}
