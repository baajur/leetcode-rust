struct Solution;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        use std::cmp::{max, Reverse};
        use std::collections::BinaryHeap;
        let mut bh = BinaryHeap::new();
        let MOD = 1_000_000_007;
        let mut se: Vec<(i64, i64)> = speed
            .into_iter()
            .zip(efficiency.into_iter())
            .map(|(a, b)| (a as i64, b as i64))
            .collect();
        se.sort_unstable_by(|a, b| b.1.cmp(&a.1)); // sort by efficiency

        let mut ret = 0i64;
        let mut sum_speed = 0i64;
        for i in 0..n as usize {
            if bh.len() < k as usize {
                bh.push(Reverse(se[i].0)); // min heap
                sum_speed += se[i].0;
            } else {
                let Reverse(min_in_heap) = bh.peek().unwrap();
                // if the minimum value in the heap is less than current worker's speed, then update sum_speed and the heap
                if min_in_heap < &se[i].0 {
                    let Reverse(p) = bh.pop().unwrap();
                    bh.push(Reverse(se[i].0));
                    sum_speed -= p;
                    sum_speed += se[i].0;
                }
            }
            ret = max(ret, se[i].1 * sum_speed);
        }
        (ret % MOD) as i32
    }
}

fn main() {
    Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2);
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_performance() {
        assert_eq!(
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2),
            60
        );
        assert_eq!(
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3),
            68
        );
        assert_eq!(
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 4),
            72
        );
    }
}
