struct KthLargest {
    k: usize,
    stream: Vec<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut stream = nums;
        stream.sort();
        stream.reverse();
        Self {
            stream,
            k: k as usize,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        let insert_index = {
            let mut ok = self.stream.len() as i32;
            let mut ng = -1;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                if self.stream[mid as usize] <= val {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok as usize
        };
        self.stream.insert(insert_index, val);
        self.stream[self.k - 1]
    }
}
fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_largest() {
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
        assert_eq!(kth.add(4), 8);

        let mut kth = KthLargest::new(1, vec![]);
        assert_eq!(kth.add(3), 3);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 10);
        assert_eq!(kth.add(9), 10);
        assert_eq!(kth.add(4), 10);
    }
}
