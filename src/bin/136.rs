struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for i in 0..nums.len() {
            if !set.contains(&nums[i]) {
                set.insert(nums[i]);
            } else {
                set.remove(&nums[i]);
            }
        }
        let mut it = set.into_iter();
        it.next().unwrap()
    }

    pub fn bit_manipulation(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            ans ^= nums[i];
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
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![2,2,1]), 1);
        assert_eq!(Solution::single_number(vec![4,1,2,1,2]), 4);
    }

    #[test]
    fn test_bit_manipulation() {
        assert_eq!(Solution::bit_manipulation(vec![2,2,1]), 1);
        assert_eq!(Solution::bit_manipulation(vec![4,1,2,1,2]), 4);
    }
}
