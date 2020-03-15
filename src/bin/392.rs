struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let cs: Vec<_> = s.chars().collect();
        let ct: Vec<_> = t.chars().collect();
        use std::collections::HashMap;
        let mut char_appear = HashMap::new();

        for i in 0..ct.len() {
            let entry = char_appear.entry(ct[i]).or_insert(vec![]);
            entry.push(i);
        }

        let mut ret = true;
        let mut cur_index: i32 = -1;
        for i in 0..cs.len() {
            let ch = cs[i];
            if let Some(pos) = char_appear.get(&ch) {
                let pos_idx = {
                    let mut ng = -1;
                    let mut ok = pos.len() as i32;
                    while ok - ng > 1 {
                        let mid = (ok + ng) / 2;
                        if pos[mid as usize] as i32 > cur_index {
                            ok = mid;
                        } else {
                            ng = mid;
                        }
                    }
                    ok as usize
                };
                if pos_idx >= pos.len() {
                    ret = false;
                    break;
                } else {
                    cur_index = pos[pos_idx] as i32;
                }
            } else {
                ret = false;
                break;
            }
        }
        ret
    }

    pub fn is_subsequence2(s: String, t: String) -> bool {
        let cs: Vec<char> = s.chars().collect();
        let ct: Vec<char> = t.chars().collect();
        let mut index_s = 0;
        let mut index_t = 0;
        while index_s < cs.len() {
            while index_t < ct.len() && ct[index_t] != cs[index_s] {
                index_t += 1;
            }
            if index_t == ct.len() {
                return false;
            }
            index_s += 1;
            index_t += 1;
        }
        true
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        assert!(Solution::is_subsequence(
            "abc".to_string(),
            "ahbgdc".to_string()
        ));
        assert!(!Solution::is_subsequence(
            "axc".to_string(),
            "ahbgdc".to_string()
        ));
        assert!(Solution::is_subsequence(
            "".to_string(),
            "ahbgdc".to_string()
        ));
        assert!(!Solution::is_subsequence(
            "acd".to_string(),
            "ahbgdc".to_string()
        ));
        assert!(Solution::is_subsequence(
            "ahg".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test_is_subsequence2() {
        assert!(Solution::is_subsequence2(
            "abc".to_string(),
            "ahbgdc".to_string()
        ));
        assert!(!Solution::is_subsequence2(
            "axc".to_string(),
            "ahbgdc".to_string()
        ));
        assert!(Solution::is_subsequence2(
            "".to_string(),
            "ahbgdc".to_string()
        ));
        assert!(!Solution::is_subsequence2(
            "acd".to_string(),
            "ahbgdc".to_string()
        ));
        assert!(Solution::is_subsequence2(
            "ahg".to_string(),
            "ahbgdc".to_string()
        ));
    }
}
