struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }
        let mut min_before = prices[0];
        let mut ret = 0;
        use std::cmp::{max, min};
        for i in 1..prices.len() {
            ret = max(ret, prices[i] - min_before);
            min_before = min(min_before, prices[i]);
        }
        ret
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![]), 0);
        assert_eq!(Solution::max_profit(vec![7]), 0);
        assert_eq!(Solution::max_profit(vec![2, 1]), 0);
        assert_eq!(Solution::max_profit(vec![2, 1, 2]), 1);
    }
}
