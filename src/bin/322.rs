struct Solution;

impl Solution {
    // Dynamic programming
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        use std::cmp::min;
        // dp[i] := the minimum number of coins needed for the amount of `i`.
        // If `i` cannot be made up by any combination of the coins, dp[i] is INF.
        let inf = 1 << 30;
        let mut dp = vec![inf; amount as usize + 1];

        dp[0] = 0;
        for i in 0..=(amount as usize) {
            if dp[i] == inf {
                continue;
            }
            for &coin in coins.iter() {
                match coin.checked_add(i as i32) {
                    Some(v) => {
                        if v > amount {
                            continue;
                        }
                    }
                    None => continue,
                };
                dp[i + coin as usize] = min(dp[i + coin as usize], dp[i] + 1);
            }
        }

        match dp[amount as usize] {
            ans if ans != inf => ans,
            _ => -1,
        }
    }

    // breadth first search: 120ms, too slow!
    pub fn bfs_coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        use std::collections::{HashSet, VecDeque};
        let mut q = VecDeque::new();
        let mut set = HashSet::new();
        for i in 0..coins.len() {
            q.push_back((coins[i], 0, 0));
        }

        while !q.is_empty() {
            let (coin, current_amount, num_coins) = q.pop_front().unwrap();
            if current_amount + coin == amount {
                return num_coins + 1;
            }
            if current_amount + coin > amount || set.contains(&(current_amount + coin)) {
                continue;
            }
            set.insert(current_amount + coin);
            for i in 0..coins.len() {
                if current_amount + coin + coins[i] > amount {
                    continue;
                }
                q.push_back((coins[i], current_amount + coin, num_coins + 1));
            }
        }

        -1
    }
}

fn main() {
    Solution::coin_change(vec![1, 2, 5], 100);
    Solution::coin_change(vec![1, 2, 5], 11);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_coin_change() {
        let test_cases = [
            (vec![1, 2, 5], 11, 3),
            (vec![2], 3, -1),
            (vec![2, 5, 7, 10], 14, 2),
            (vec![2, 3, 4, 8, 10, 14], 0, 0),
            (vec![1, 2, 5], 10, 2),
            (vec![1, 2147483647], 2, 2),
        ];

        for (arg1, arg2, expected) in test_cases.iter() {
            // assert_eq!(Solution::bfs_coin_change(arg1.clone(), *arg2), *expected);
            assert_eq!(Solution::coin_change(arg1.clone(), *arg2), *expected);
        }
    }
}
