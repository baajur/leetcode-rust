struct Solution;

use std::cmp::min;
use std::collections::HashMap;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut t_count = t.chars().fold(HashMap::new(), |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        });

        let s: Vec<char> = s.chars().collect();
        let mut s_count = HashMap::new();
        let mut right = 0;
        let mut min_w = (0, s.len() + 1);
        for left in 0..s.len() {
            while right < s.len() && !is_included(&t_count, &s_count) {
                *s_count.entry(s[right]).or_insert(0) += 1;
                right += 1;
            }

            if !is_included(&t_count, &s_count) {
                break;
            }
            if min_w.1 - min_w.0 > right - left {
                min_w = (left, right);
            }

            if left == right {
                right += 1;
            } else {
                s_count.entry(s[left]).and_modify(|c| *c -= 1);
            }
        }

        if min_w.1 == s.len() + 1 {
            "".to_string()
        } else {
            s.get(min_w.0..min_w.1).unwrap().iter().collect()
        }
    }
}

fn is_included(t_count: &HashMap<char, i32>, s_count: &HashMap<char, i32>) -> bool {
    t_count.iter().all(|(ch, count)| match s_count.get(ch) {
        Some(x) => x >= count,
        None => false,
    })
}

fn main() {
    dbg!(Solution::min_window(
        "ADOBECODEBANC".to_string(),
        "ABC".to_string()
    ));
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_included() {
        let t = {
            let mut t = HashMap::new();
            t.insert('a', 1);
            t
        };
        let s = {
            let mut s = HashMap::new();
            s.insert('a', 1);
            s
        };
        assert!(is_included(&t, &s));
    }

    #[test]
    fn test_min_window() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
        assert_eq!(
            Solution::min_window("XYZWUV".to_string(), "ABC".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::min_window("XYZWUV".to_string(), "WZV".to_string()),
            "ZWUV".to_string()
        );
        assert_eq!(
            Solution::min_window("XYZWUV".to_string(), "YX".to_string()),
            "XY".to_string()
        );
        assert_eq!(
            Solution::min_window("ABC".to_string(), "C".to_string()),
            "C".to_string()
        );
        assert_eq!(
            Solution::min_window("abc".to_string(), "a".to_string()),
            "a".to_string()
        );
        assert_eq!(
            Solution::min_window("abc".to_string(), "b".to_string()),
            "b".to_string()
        );
    }
}
