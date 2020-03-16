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
}
