struct Solution;

impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut ret = None;
        let mut right = 0;
        let mut cur_sum = 0;
        for left in 0..nums.len() {
            while right < nums.len() && cur_sum + nums[right] < s {
                cur_sum += nums[right];
                right += 1;
            }
            if right == nums.len() && cur_sum < s {
                break;
            }
            ret = match ret {
                None => Some(right + 1 - left),
                Some(prev) => Some(std::cmp::min(prev, right + 1 - left)),
            };

            if right == left {
                right += 1;
            } else {
                cur_sum -= nums[left];
            }
        }
        match ret {
            None => 0,
            Some(x) => x as i32,
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
    fn test_min_sub_array_len() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(1, vec![2, 3, 1, 2, 4, 3]), 1);
        assert_eq!(Solution::min_sub_array_len(1, vec![]), 0);
        assert_eq!(Solution::min_sub_array_len(10, vec![1, 2, 3]), 0);
    }
}
