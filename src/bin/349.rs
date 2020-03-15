struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let nums1: HashSet<_> = nums1.into_iter().collect();
        let nums2: HashSet<_> = nums2.into_iter().collect();
        nums1.intersection(&nums2).map(|x| *x).collect()
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
        assert_eq!(
            Solution::intersection(vec![1, 2, 3], vec![9, 4, 9, 8, 4]),
            vec![]
        );
    }
}
