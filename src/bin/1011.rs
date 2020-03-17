struct Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let mut acc_sum = Vec::with_capacity(weights.len() + 1);
        acc_sum.push(0);
        let mut max_w = 0;
        use std::cmp::max;
        for i in 0..weights.len() {
            max_w = max(max_w, weights[i]);
            let last = acc_sum[i];
            acc_sum.push(last + weights[i]);
        }

        let min_cap = {
            let mut ok = acc_sum[weights.len()];
            let mut ng = max_w - 1;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                if shippable(&acc_sum, d, mid) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        };

        min_cap
    }
}

fn shippable(acc: &[i32], d: i32, cap: i32) -> bool {
    // do not pass cap that is smaller than max of weights.
    // it will cause infinite loop
    let mut i = 0;
    let mut day = 0;
    while i < acc.len() - 1 {
        i = {
            let mut ok = i;
            let mut ng = acc.len();
            while ng - ok > 1 {
                let mid = (ok + ng) / 2;
                if acc[mid] - acc[i] <= cap {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            dbg!(ok, i);
            ok
        };
        day += 1;
    }
    day <= d
}
fn main() {
    let acc = vec![0, 2, 4, 7, 15, 25];
    shippable(&acc, 3, 10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shippable() {
        let acc = vec![0, 2, 4, 7, 15, 25];
        assert!(shippable(&acc, 3, 10));
        assert!(!shippable(&acc, 2, 10));

        let acc = vec![0, 2, 4, 7, 15, 25];
        assert!(shippable(&acc, 6, 10));
    }

    #[test]
    fn test_ship_within_days() {
        assert_eq!(
            Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
            15
        );
        assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
        assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
        assert_eq!(Solution::ship_within_days(vec![300], 1), 300);
    }
}
