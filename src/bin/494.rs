struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        dfs(&nums, s)
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
}
