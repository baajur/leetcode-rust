struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut prev_sum = 0;
        let mut max_sum = std::i32::MIN;
        for i in 0..nums.len() {
            prev_sum = nums[i] + max(0, prev_sum); // discard preg_sum if it's negative
            max_sum = max(max_sum, prev_sum);
        }
        max_sum
    }

    pub fn my_max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }

        use std::cmp::{max, min};
        let mut sums = Vec::new();
        sums.push(0);
        let mut cur = (0, 0);
        let mut ans = std::i32::MIN;

        for i in 0..nums.len() {
            let prev = sums.last().unwrap();
            let sum = nums[i] + prev;
            sums.push(sum);
            let next_max = max(cur.0, sum);
            if cur.1 > sum {
                // when updating min, we have to reset max value as well
                cur = (std::i32::MIN, sum);
                ans = max(ans, nums[i]);
            } else {
                cur = (max(cur.0, sum), min(cur.1, sum));
                ans = max(ans, cur.0 - cur.1);
            }
        }
        ans = max(ans, cur.0 - cur.1);

        ans
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );

        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, 0, 4, -1, 2, 1, -5, 5]),
            7
        );

        assert_eq!(Solution::max_sub_array(vec![-1]), -1);
        assert_eq!(Solution::max_sub_array(vec![-1, -2, -3]), -1);
        assert_eq!(Solution::max_sub_array(vec![-2, -1, -3]), -1);
        assert_eq!(Solution::max_sub_array(vec![1, 2]), 3);
    }

    #[test]
    fn test_my_max_sub_array() {
        assert_eq!(
            Solution::my_max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );

        assert_eq!(
            Solution::my_max_sub_array(vec![-2, 1, 0, 4, -1, 2, 1, -5, 5]),
            7
        );

        assert_eq!(Solution::my_max_sub_array(vec![-1]), -1);
        assert_eq!(Solution::my_max_sub_array(vec![-1, -2, -3]), -1);
        assert_eq!(Solution::my_max_sub_array(vec![-2, -1, -3]), -1);
        assert_eq!(Solution::my_max_sub_array(vec![1, 2]), 3);
    }
}
