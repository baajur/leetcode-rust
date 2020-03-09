struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::{BTreeMap, HashMap};
        let mut freq = HashMap::new();
        for i in 0..nums.len() {
            *freq.entry(nums[i]).or_insert(0) += 1;
        }
        let mut elem_per_freq = BTreeMap::new();
        for (key, value) in freq.into_iter() {
            let entry = elem_per_freq.entry(Reverse(value)).or_insert(vec![]);
            entry.push(key);
        }
        let mut ans = Vec::new();
        'outer: for (_, elems) in elem_per_freq.into_iter().take(k as usize) {
            for e in elems {
                ans.push(e);
                if ans.len() == k as usize {
                    break 'outer;
                }
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
    fn test_top_k_frequent() {
        let tests = [
            (vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2]),
            // (vec![1, 1, 1, 2, 2, 3, 3, 4], 2, vec![1, 2]), // sometimes output become vec![1, 3]
            (vec![1], 1, vec![1]),
            // (vec![4, 1, -1, 2, -1, 2, 3], 2, vec![-1, 2]), // sometimes output become vec![2, -1]
        ];
        for test in tests.iter() {
            let nums = test.0.clone();
            let k = test.1;
            let expected = test.2.clone();
            assert_eq!(Solution::top_k_frequent(nums, k), expected);
        }
    }
}
