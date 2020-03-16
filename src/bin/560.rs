struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cum_sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            cum_sum[i + 1] = cum_sum[i] + nums[i];
        }
        dbg!(&cum_sum);

        let mut ret = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if cum_sum[j + 1] - cum_sum[i] == k {
                    ret += 1;
                }
            }
        }
        ret
    }

    pub fn subarray_sum2(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut count = 0;
        let mut sum = 0;
        map.insert(0, 1);
        for i in 0..nums.len() {
            sum += nums[i];
            if let Some(x) = map.get(&(sum - k)) {
                count += x;
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        count
    }
}

fn main() {
    dbg!(Solution::subarray_sum(vec![1, 1, 1], 2));
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subarray_sum() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![], 4), 0);
        assert_eq!(Solution::subarray_sum(vec![1], 4), 0);
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1, 2, 1], 3), 3);
        assert_eq!(Solution::subarray_sum(vec![1], 0), 0);
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 3), 1);
        assert_eq!(
            Solution::subarray_sum(vec![28, 54, 7, -70, 22, 65, -6], 100),
            1
        );
    }

    #[test]
    fn test_subarray_sum2() {
        assert_eq!(Solution::subarray_sum2(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum2(vec![], 4), 0);
        assert_eq!(Solution::subarray_sum2(vec![1], 4), 0);
        assert_eq!(Solution::subarray_sum2(vec![1, 1, 1, 2, 1], 3), 3);
        assert_eq!(Solution::subarray_sum2(vec![1], 0), 0);
        assert_eq!(Solution::subarray_sum2(vec![1, 1, 1], 3), 1);
        assert_eq!(
            Solution::subarray_sum2(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0),
            55
        );
        assert_eq!(
            Solution::subarray_sum2(vec![28, 54, 7, -70, 22, 65, -6], 100),
            1
        );
    }
}
