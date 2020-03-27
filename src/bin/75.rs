struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        let mut right_most = 0;
        let mut left_most = nums.len() - 1;
        let mut cur = 0;
        while cur <= left_most {
            if nums[cur] == 0 {
                nums.swap(cur, right_most);
                right_most += 1;
                cur += 1;
            } else if nums[cur] == 2 {
                nums.swap(cur, left_most);
                if left_most == 0 {
                    return;
                }
                left_most -= 1;
            } else {
                cur += 1;
            }
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
    fn test_sort_colors() {
        let mut v = vec![1, 2, 0, 1, 1, 2, 0, 0, 0, 1, 2];
        Solution::sort_colors(&mut v);
        assert_eq!(v, vec![0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2]);

        let mut v = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut v);
        assert_eq!(v, vec![0, 0, 1, 1, 2, 2]);

        let mut v = vec![];
        Solution::sort_colors(&mut v);
        assert_eq!(v, vec![]);

        let mut v = vec![2];
        Solution::sort_colors(&mut v);
        assert_eq!(v, vec![2]);

        let mut v = vec![2, 2];
        Solution::sort_colors(&mut v);
        assert_eq!(v, vec![2, 2]);

        let mut v = vec![2; 10];
        Solution::sort_colors(&mut v);
        assert_eq!(v, vec![2; 10]);
    }
}
