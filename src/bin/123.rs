struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }
        // dp[i][j] := max profit before j-th stock with i transactions being done
        let mut dp = vec![vec![0; prices.len()]; 3];
        for i in 1..3 {
            for j in 0..prices.len() {
                let mut max_on_transaction = 0;
                for k in 0..j {
                    let prev = if k == 0 { 0 } else { dp[i - 1][k - 1] };
                    max_on_transaction =
                        std::cmp::max(max_on_transaction, prices[j] - prices[k] + prev);
                }
                dp[i][j] = std::cmp::max(if j == 0 { 0 } else { dp[i][j - 1] }, max_on_transaction);
            }
        }

        dp[2][prices.len() - 1]
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
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![]), 0);
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]), 13);
    }
}
