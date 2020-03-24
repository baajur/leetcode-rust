struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s
            .chars()
            .filter_map(|c| {
                if c.is_ascii_alphanumeric() {
                    Some(c.to_ascii_lowercase())
                } else {
                    None
                }
            })
            .collect();
        let mut rev_s = s.clone();
        rev_s.reverse();

        s == rev_s
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
        assert!(Solution::is_palindrome(
            "A man, !!!!!!!!!!!!!.a plan, a canal:;;;; Panama?????".to_string()
        ));
        assert!(Solution::is_palindrome("".to_string()));
        assert!(!Solution::is_palindrome("race a car".to_string()));
    }
}
