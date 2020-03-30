struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s: Vec<_> = s.chars().collect();
        let p: Vec<_> = p.chars().collect();
        if s.len() < p.len() {
            return vec![];
        }
        let mut char_counts = vec![0; 26];

        let mut p_counts = vec![0; 26];
        for i in 0..p.len() {
            p_counts[char2index(p[i])] += 1;
        }

        let mut ans = Vec::new();

        for i in 0..(p.len() - 1) {
            char_counts[char2index(s[i])] += 1;
        }

        for i in (p.len() - 1)..s.len() {
            char_counts[char2index(s[i])] += 1;
            let begin = i - (p.len() - 1);
            if identical(&char_counts, &p_counts) {
                ans.push(begin as i32);
            }
            char_counts[char2index(s[begin])] -= 1;
        }

        ans
    }
}

fn char2index(c: char) -> usize {
    (c as u8 - 'a' as u8) as usize
}

fn identical(cur: &[i32], p: &[i32]) -> bool {
    for i in 0..26 {
        if cur[i] != p[i] {
            return false;
        }
    }
    true
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_anagrams() {
        assert_eq!(
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
            vec![0, 6]
        );
        assert_eq!(
            Solution::find_anagrams("abab".to_string(), "ab".to_string()),
            vec![0, 1, 2]
        );
        assert!(Solution::find_anagrams("".to_string(), "ab".to_string()).is_empty());
    }
}
