struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }
        let mut ret = 0;
        let mut valley = prices[0];
        let mut peek = prices[0];
        let mut i = 0;
        while i < prices.len() - 1 {
            while i < prices.len() - 1 && prices[i] >= prices[i + 1] {
                i += 1;
            }
            valley = prices[i];
            while i < prices.len() - 1 && prices[i] <= prices[i + 1] {
                i += 1;
            }
            peek = prices[i];
            ret += peek - valley;
            valley = peek;
        }
        ret
    }

    pub fn simplest_max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }
        let mut ret = 0;
        for i in 0..(prices.len() - 1) {
            if prices[i] < prices[i + 1] {
                ret += prices[i + 1] - prices[i];
            }
        }
        ret
    }

    pub fn brute_force_max_profit(prices: Vec<i32>) -> i32 {
        calc(&prices, 0)
    }
}

fn calc(prices: &[i32], s: usize) -> i32 {
    use std::cmp::max;
    if s >= prices.len() {
        return 0;
    }
    let mut ret = 0;
    for buy in s..prices.len() {
        for sell in (buy + 1)..prices.len() {
            if prices[buy] >= prices[sell] {
                continue;
            }
            ret = max(ret, calc(prices, sell + 1) + prices[sell] - prices[buy]);
        }
    }
    ret
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![]), 0);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 5, 10, 20]), 19);
    }
    #[test]
    fn test_simplest_max_profit() {
        assert_eq!(Solution::simplest_max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::simplest_max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::simplest_max_profit(vec![]), 0);
        assert_eq!(Solution::simplest_max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::simplest_max_profit(vec![1, 5, 10, 20]), 19);
    }
    #[test]
    fn test_brute_force_max_profit() {
        assert_eq!(Solution::brute_force_max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::brute_force_max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::brute_force_max_profit(vec![]), 0);
        assert_eq!(Solution::brute_force_max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::brute_force_max_profit(vec![1, 5, 10, 20]), 19);
    }
}
