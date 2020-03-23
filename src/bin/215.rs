struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_by(|a, b| b.cmp(&a));
        nums[k as usize - 1]
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(Solution::find_kth_largest(vec![1], 1), 1);
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
        assert_eq!(Solution::find_kth_largest(vec![4, 4, 4, 4, 4, 4], 4), 4);
    }
}
