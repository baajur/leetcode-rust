struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut last_non_zero_at = 0;
        for i in 0..len {
            if nums[i] != 0 {
                nums[last_non_zero_at] = nums[i];
                last_non_zero_at += 1;
            }
        }
        for i in last_non_zero_at..len {
            nums[i] = 0;
        }
    }
}

fn main() {
    let mut v = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut v);
    dbg!(&v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeros() {
        let mut v = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![1, 3, 12, 0, 0]);

        let mut v = vec![0, 0, 0];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![0, 0, 0]);

        let mut v = vec![];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![]);

        let mut v = vec![1];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![1]);

        let mut v = vec![0, 0, 0, 0, 1];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![1, 0, 0, 0, 0]);

        let mut v = vec![1, 2, 3, 4, 5];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }
}
