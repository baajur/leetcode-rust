struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() <= 2 {
            return false;
        }
        let mut first = std::i32::MAX;
        let mut second = std::i32::MAX;

        for i in 0..nums.len() {
            if nums[i] <= first {
                first = nums[i];
            } else if nums[i] <= second {
                second = nums[i];
            } else {
                return true;
            }
        }

        false
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increasing_triplet() {
        assert!(Solution::increasing_triplet(vec![1, 2, 3]));
        assert!(Solution::increasing_triplet(vec![5, 2, 4, 1, 0, 5]));
        assert!(!Solution::increasing_triplet(vec![2, 1, 3]));
        assert!(!Solution::increasing_triplet(vec![]));
        assert!(!Solution::increasing_triplet(vec![1]));
        assert!(!Solution::increasing_triplet(vec![1, 2]));
    }
}
