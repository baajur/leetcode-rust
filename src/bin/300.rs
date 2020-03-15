struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let mut dp = vec![vec![-1; nums.len()]; nums.len()];
        dp[0][0] = 1;

        use std::cmp::{max, min};
        for i in 1..nums.len() {
            for j in 0..=i {
                if j < i {
                    dp[i][j] = max(dp[i][j], dp[i - 1][j]); // not choose nums[i]
                }
                if nums[j] < nums[i] {
                    dp[i][i] = max(dp[i][i], dp[i - 1][j] + 1); // choose nums[i+1]
                } else {
                    dp[i][i] = max(dp[i][i], 1);
                }
            }
        }

        let mut ret = -1;
        for i in 0..nums.len() {
            ret = max(ret, dp[nums.len() - 1][i]);
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
    fn test_length_of_lis() {
        assert_eq!(Solution::length_of_lis(vec![2, 1, 5, 1, 6]), 3);
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![9, 8, 7, 6, 5]), 1);
        assert_eq!(Solution::length_of_lis(vec![9]), 1);
        assert_eq!(Solution::length_of_lis(vec![10, 5, 2, 6, 3, 9]), 3);
        assert_eq!(Solution::length_of_lis(vec![]), 0);
    }
}
