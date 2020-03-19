struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 2 {
            return vec![];
        }

        use std::collections::HashSet;
        let mut ans = HashSet::new();

        for i in 0..(nums.len() - 2) {
            let x = nums[i];
            // find y, z that sum up to -x
            let mut appear = HashSet::new();
            for j in (i + 1)..nums.len() {
                if appear.contains(&nums[j]) {
                    let mut v = vec![x, nums[j], -x - nums[j]];
                    v.sort();
                    ans.insert(v);
                } else {
                    appear.insert(-x - nums[j]);
                }
            }
        }
        ans.into_iter().collect()
    }

    pub fn tle_three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 2 {
            return vec![];
        }

        use std::collections::HashSet;
        let mut triplets = HashSet::new();

        for i in 0..(nums.len() - 2) {
            for j in (i + 1)..(nums.len() - 1) {
                for k in (j + 1)..nums.len() {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        let mut tr = vec![nums[i], nums[j], nums[k]];
                        tr.sort();
                        triplets.insert(tr);
                    }
                }
            }
        }

        triplets.into_iter().collect()
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_sum() {
        assert!(is_same_set(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, 0, 1], vec![-1, -1, 2]]
        ));
        assert!(is_same_set(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4, -1]),
            vec![vec![-1, 0, 1], vec![-1, -1, 2]]
        ));
        assert!(Solution::three_sum(vec![0]).is_empty());
        assert!(Solution::three_sum(vec![]).is_empty());
    }

    #[test]
    fn tle_three_sum() {
        assert!(is_same_set(
            Solution::tle_three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, 0, 1], vec![-1, -1, 2]]
        ));
        assert!(is_same_set(
            Solution::tle_three_sum(vec![-1, 0, 1, 2, -1, -4, -1]),
            vec![vec![-1, 0, 1], vec![-1, -1, 2]]
        ));
        assert!(Solution::tle_three_sum(vec![0]).is_empty());
        assert!(Solution::tle_three_sum(vec![]).is_empty());
    }

    fn is_same_set(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> bool {
        use std::collections::HashSet;
        a.into_iter().collect::<HashSet<_>>() == b.into_iter().collect::<HashSet<_>>()
    }
}
