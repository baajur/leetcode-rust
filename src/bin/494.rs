struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        dfs(&nums, s)
    }

    pub fn dp_find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        if nums.is_empty() || s.abs() > 1000 {
            return 0;
        }

        let mut dp = vec![vec![0; 2020]; nums.len() + 2];
        let target = 1010 + s as usize;
        dp[0][(1010 + nums[0]) as usize] += 1;
        dp[0][(1010 - nums[0]) as usize] += 1;

        for i in 1..nums.len() {
            for j in 0..2020 {
                if j > nums[i] as usize {
                    dp[i][j] += dp[i - 1][j - nums[i] as usize];
                }
                if j + (nums[i] as usize) < 2020 {
                    dp[i][j] += dp[i - 1][j + nums[i] as usize];
                }
            }
        }

        dp[nums.len() - 1][target]
    }
}

fn dfs(nums: &[i32], rest: i32) -> i32 {
    if nums.len() == 1 {
        if rest == nums[0] && rest == -nums[0] {
            return 2;
        } else if rest == nums[0] || rest == -nums[0] {
            return 1;
        } else {
            return 0;
        }
    }

    dfs(&nums[1..], rest + nums[0]) + dfs(&nums[1..], rest - nums[0])
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_target_sum_ways() {
        assert_eq!(Solution::find_target_sum_ways(vec![1; 5], 3), 5);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 0], 1), 2);
    }

    #[test]
    fn test_dp_find_target_sum_ways() {
        assert_eq!(Solution::dp_find_target_sum_ways(vec![1; 5], 3), 5);
        assert_eq!(Solution::dp_find_target_sum_ways(vec![1, 0], 1), 2);
        assert_eq!(
            Solution::dp_find_target_sum_ways(vec![0, 0, 0, 0, 0, 0, 0, 0, 1], 1),
            256
        );
        assert_eq!(
            Solution::dp_find_target_sum_ways(vec![1, 2, 7, 9, 981], 1000000000),
            0
        );
        assert_eq!(Solution::dp_find_target_sum_ways(vec![], 1), 0);
    }
}
