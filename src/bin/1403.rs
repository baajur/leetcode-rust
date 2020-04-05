struct Solution;

impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        nums.reverse();
        let mut ans = Vec::new();
        let total = nums.iter().sum::<i32>();
        let mut cur_sum = 0;
        for i in 0..nums.len() {
            if 2 * cur_sum > total {
                break;
            }
            ans.push(nums[i]);
            cur_sum += nums[i];
        }
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
    fn test_min_subsequence() {
        assert_eq!(Solution::min_subsequence(vec![4, 3, 10, 9, 8]), vec![10, 9]);
        assert_eq!(Solution::min_subsequence(vec![1]), vec![1]);
        assert_eq!(
            Solution::min_subsequence(vec![4, 4, 7, 6, 7]),
            vec![7, 7, 6]
        );
    }
}
