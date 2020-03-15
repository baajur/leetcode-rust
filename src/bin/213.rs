struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        use std::cmp::max;
        if nums.len() == 2 {
            return max(nums[0], nums[1]);
        }

        let mut dp = vec![vec![0; 2]; nums.len()];
        dp[0][0] = 0;
        dp[0][1] = nums[0];
        dp[1][0] = nums[1];
        dp[1][1] = dp[0][1];

        for i in 2..nums.len() {
            if i != nums.len() - 1 {
                dp[i][0] = max(dp[i - 1][0], dp[i - 2][0] + nums[i]);
                dp[i][1] = max(dp[i - 1][1], dp[i - 2][1] + nums[i]);
            } else {
                dp[i][1] = dp[i - 1][1];
                dp[i][0] = max(dp[i - 1][0], dp[i - 2][0] + nums[i]);
            }
        }
        let l = nums.len();
        max(dp[l - 1][0], dp[l - 1][1])
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
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![4, 2, 6, 9]), 11);
    }
}
