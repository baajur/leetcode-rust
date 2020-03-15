struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        use std::collections::HashSet;
        let mut once = HashSet::new();
        let mut twice = HashSet::new();
        let c = s.chars().collect::<Vec<_>>();
        for i in 0..c.len() {
            let ch = c[i];
            if twice.contains(&ch) {
                continue;
            }
            if once.contains(&c[i]) {
                twice.insert(ch);
                once.remove(&ch);
                continue;
            }
            once.insert(ch);
        }

        if once.is_empty() {
            -1
        } else {
            for i in 0..c.len() {
                if once.contains(&c[i]) {
                    return i as i32;
                }
            }
            unreachable!();
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
    fn test_first_uniq_char() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
        assert_eq!(Solution::first_uniq_char("aabbcc".to_string()), -1);
        assert_eq!(Solution::first_uniq_char("".to_string()), -1);
    }
}
