struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        use std::cmp::{max, min};
        let mut ans = -1;
        while left < right {
            ans = max(
                ans,
                (right - left) as i32 * min(height[left], height[right]),
            );
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
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
    fn max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 2]), 1);
        assert_eq!(Solution::max_area(vec![3, 9, 3, 4, 7, 2, 12, 6]), 45);
    }
}
