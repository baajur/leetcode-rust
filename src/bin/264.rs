struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashSet};
        let mut num_calculated = 0;
        let mut bh = BinaryHeap::new();
        let mut uglys = Vec::new();
        let mut already_used = HashSet::new();
        bh.push(Reverse(1i64));
        already_used.insert(1i64);

        while uglys.len() < n as usize {
            let Reverse(min_in_heap) = bh.pop().unwrap();
            uglys.push(min_in_heap);
            for &mul in [2, 3, 5].iter() {
                if let Some(val) = min_in_heap.checked_mul(mul) {
                    if !already_used.contains(&val) {
                        already_used.insert(val);
                        bh.push(Reverse(min_in_heap * mul));
                    }
                }
            }
        }

        uglys[n as usize - 1] as i32
    }
}

fn main() {
    dbg!(Solution::nth_ugly_number(27));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_ugly_number() {
        let test_cases = [
            (1, 1),
            (10, 12),
            (11, 15),
            (12, 16),
            (27, 64),
            (1407, 536870912),
        ];
        for &(input, expected) in test_cases.iter() {
            assert_eq!(Solution::nth_ugly_number(input), expected);
        }
    }
}
