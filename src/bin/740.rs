struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }

        let mut nums = nums;
        nums.sort_unstable();
        let mut dp = vec![0; nums[nums.len() - 1] as usize + 1];
        let mut idx = 0;
        for i in 1..dp.len() {
            let mut s = 0;
            while idx < nums.len() && nums[idx] as usize == i {
                s += i;
                idx += 1;
            }
            if i >= 2 {
                dp[i] = std::cmp::max(dp[i - 2] + s, dp[i - 1]);
            } else {
                dp[i] = s;
            }
        }
        *dp.last().unwrap() as i32
    }

    pub fn delete_and_earn2(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; 10001];
        for i in 0..nums.len() {
            dp[nums[i] as usize] += nums[i];
        }
        for i in 2..10001 {
            dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + dp[i]);
        }
        dp[10000]
    }
}

fn main() {
    dbg!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]));
    dbg!(Solution::delete_and_earn(vec![3, 4, 2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_and_earn() {
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
        assert_eq!(Solution::delete_and_earn(vec![3, 2, 4]), 6);
        assert_eq!(Solution::delete_and_earn(vec![]), 0);
        assert_eq!(Solution::delete_and_earn(vec![2]), 2);
        assert_eq!(Solution::delete_and_earn(vec![2; 10]), 20);
        assert_eq!(
            Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4, 5, 5, 6, 6, 6, 7, 7]),
            33
        );
        assert_eq!(
            Solution::delete_and_earn(vec![1, 1, 1, 2, 4, 5, 5, 5, 6]),
            18
        );
    }

    #[test]
    fn test_delete_and_earn2() {
        assert_eq!(Solution::delete_and_earn2(vec![2, 2, 3, 3, 3, 4]), 9);
        assert_eq!(Solution::delete_and_earn2(vec![3, 2, 4]), 6);
        assert_eq!(Solution::delete_and_earn2(vec![]), 0);
        assert_eq!(Solution::delete_and_earn2(vec![2]), 2);
        assert_eq!(Solution::delete_and_earn2(vec![2; 10]), 20);
        assert_eq!(
            Solution::delete_and_earn2(vec![2, 2, 3, 3, 3, 4, 5, 5, 6, 6, 6, 7, 7]),
            33
        );
        assert_eq!(
            Solution::delete_and_earn2(vec![1, 1, 1, 2, 4, 5, 5, 5, 6]),
            18
        );
    }
}
