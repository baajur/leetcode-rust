struct Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        use std::collections::BTreeMap;
        let mut m = BTreeMap::new();
        for i in 0..arr.len() {
            *m.entry(arr[i]).or_insert(0) += 1;
        }
        let mut ans = None;
        for (k, v) in m {
            if k == v {
                ans = Some(k);
            }
        }
        match ans {
            Some(x) => x,
            None => -1,
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
    fn test_find_lucky() {
        assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
        assert_eq!(Solution::find_lucky(vec![1]), 1);
        assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
        assert_eq!(Solution::find_lucky(vec![2, 2, 2, 3, 3]), -1);
        assert_eq!(Solution::find_lucky(vec![5]), -1);
        assert_eq!(Solution::find_lucky(vec![7; 7]), 7);
    }
}
