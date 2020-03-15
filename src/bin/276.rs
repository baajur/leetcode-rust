struct Solution;

impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n <= 1 {
            return k;
        }
        if n == 2 {
            return k * k;
        }
        if n >= 3 && k == 1 {
            return 0;
        }
        let mut dp = vec![vec![0; 2]; n as usize];
        dp[0][0] = k;
        dp[0][1] = 0;
        dp[1][0] = dp[0][0] * (k - 1);
        dp[1][1] = k;

        for i in 2..(n as usize) {
            dp[i][0] = (dp[i - 1][0] + dp[i - 1][1]) * (k - 1);
            dp[i][1] = dp[i - 1][0];
        }
        dp[n as usize - 1][0] + dp[n as usize - 1][1]
    }
}

fn main() {
    Solution::num_ways(3, 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_ways() {
        assert_eq!(Solution::num_ways(0, 0), 0);
        assert_eq!(Solution::num_ways(3, 2), 6);
        assert_eq!(Solution::num_ways(4, 2), 10);
        assert_eq!(Solution::num_ways(1, 2), 2);
        assert_eq!(Solution::num_ways(2, 2), 4);
        assert_eq!(Solution::num_ways(3, 1), 0);
    }
}
