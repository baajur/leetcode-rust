struct Solution;

#[derive(Eq, PartialEq)]
struct PairSum(i32, i32, i32);

impl PartialOrd for PairSum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for PairSum {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;
        let mut h = BinaryHeap::new();

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                let s = nums1[i] + nums2[j];
                h.push(PairSum(s, nums1[i], nums2[j]));
            }
        }

        let mut ret = Vec::with_capacity(k as usize);
        for _ in 0..std::cmp::min(k as usize, nums1.len() * nums2.len()) {
            let PairSum(_, u, v) = h.pop().unwrap();
            ret.push(vec![u, v]);
        }

        ret
    }
}

fn main() {
    let r = Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3);
    dbg!(r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_smallest_pairs() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );
        assert_eq!(
            Solution::k_smallest_pairs(vec![], vec![1, 2, 3], 2),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::k_smallest_pairs(vec![], vec![], 2),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 2], vec![3], 3),
            vec![vec![1, 3], vec![2, 3]]
        );
    }
}
