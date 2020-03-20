struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        solve(0, arr.len(), &arr, &mut memo)
    }
}

fn solve(
    left: usize,
    right: usize,
    arr: &Vec<i32>,
    memo: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if right - left <= 1 {
        return 0;
    }
    if let Some(&v) = memo.get(&(left, right)) {
        return v;
    }

    use std::cmp::{max, min};
    let mut ans = std::i32::MAX;
    for k in (left + 1)..right {
        ans = min(
            ans,
            solve(left, k, arr, memo)
                + solve(k, right, arr, memo)
                + arr.get(left..k).unwrap().iter().max().unwrap()
                    * arr.get(k..right).unwrap().iter().max().unwrap(),
        );
    }
    memo.insert((left, right), ans);

    ans
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mct_from_leaf_values() {
        assert_eq!(Solution::mct_from_leaf_values(vec![6, 2]), 12);
        assert_eq!(Solution::mct_from_leaf_values(vec![6, 2, 4]), 32);
        assert_eq!(Solution::mct_from_leaf_values(vec![1, 2, 3, 4]), 20);
        assert_eq!(
            Solution::mct_from_leaf_values(vec![1, 10, 11, 8, 12, 14, 1, 15, 3, 11, 6, 12]),
            1143
        );
    }
}
