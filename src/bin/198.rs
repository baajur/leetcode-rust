struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        match nums.len() {
            0 => 0,
            1 => nums[0],
            x => {
                let mut dp = vec![0; nums.len()];
                dp[0] = nums[0];
                dp[1] = max(dp[0], nums[1]);
                for i in 2..nums.len() {
                    dp[i] = max(nums[i] + dp[i - 2], dp[i - 1]);
                }
                dp[x - 1]
            }
        }
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob(vec![]), 0);
        assert_eq!(Solution::rob(vec![1]), 1);
        assert_eq!(Solution::rob(vec![1, 2]), 2);
        assert_eq!(Solution::rob(vec![1, 4, 2]), 4);
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
    }
}
