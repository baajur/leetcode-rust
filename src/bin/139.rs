struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        use std::collections::HashSet;
        let word_dict: HashSet<String> = word_dict.into_iter().collect();
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        for i in 1..=s.len() {
            for j in 0..i {
                if dp[j] && word_dict.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[s.len()]
    }
}

fn main() {
    Solution::word_break(
        "leetcode".to_string(),
        vec!["leet".to_string(), "code".to_string()],
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        assert!(Solution::word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ));
        assert!(Solution::word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()]
        ));
        assert!(!Solution::word_break(
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string()
            ]
        ));
    }
}
