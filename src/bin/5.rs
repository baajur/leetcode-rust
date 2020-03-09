struct Solution;

impl Solution {
    // Dynamic programming solution
    pub fn longest_palindrome(s: String) -> String {
        let ch: Vec<_> = s.chars().collect();
        if ch.is_empty() {
            return "".to_string();
        }
        let mut dp = vec![vec![false; s.len() + 1]; s.len() + 1];
        let mut range = (0, 1);
        // initialize
        for i in 0..ch.len() {
            dp[i][i + 1] = true;
            if i + 2 <= ch.len() && ch[i] == ch[i + 1] {
                dp[i][i + 2] = true;
                range = (i, i + 2);
            }
        }

        if ch.len() >= 3 {
            for l in 3..=ch.len() {
                for b in 0..=(ch.len() - l) {
                    // S(b, b + l) is palindrome?
                    if dp[b + 1][b + l - 1] && ch[b] == ch[b + l - 1] {
                        dp[b][b + l] = true;
                        range = (b, b + l);
                    }
                }
            }
        }

        ch.get(range.0..range.1).unwrap().iter().collect()
    }

    pub fn expand_longest_palindrome(s: String) -> String {
        let chars: Vec<_> = s.chars().collect();

        if chars.is_empty() {
            return "".to_string();
        }

        let mut range = (0, 0);
        // find the longest palindrome of which length is `odd`
        for i in 0..chars.len() {
            let mut left = i;
            let mut right = i;
            while let (Some(l), Some(r)) = (left.checked_sub(1), right.checked_add(1)) {
                let left_char = *chars.get(l).unwrap_or(&'$');
                let right_char = *chars.get(r).unwrap_or(&'-');
                if left_char == right_char {
                    left = l;
                    right = r;
                } else {
                    break;
                }
            }
            if range.1 - range.0 < right - left {
                range = (left, right);
            }
        }
        // find the longest palindrome of which length is `even`
        for i in 1..chars.len() {
            let mut left = i;
            let mut right = i;
            while let (Some(l), Some(r)) = (left.checked_sub(1), right.checked_add(1)) {
                let left_char = *chars.get(l).unwrap_or(&'$');
                let right_char = *chars.get(r - 1).unwrap_or(&'-');
                if left_char == right_char {
                    left = l;
                    right = r;
                } else {
                    break;
                }
            }
            if right == left {
                continue;
            }
            if range.1 - range.0 < right - 1 - left {
                range = (left, right - 1);
            }
        }

        chars.get(range.0..=range.1).unwrap().iter().collect()
    }

    pub fn tmp_longest_palindrome(s: String) -> String {
        let chars = {
            let ch: Vec<char> = s.chars().collect();
            let mut ret = Vec::new();
            for i in 0..ch.len() {
                match ch.get(i + 1) {
                    Some(&c) if c == ch[i] => {
                        ret.push(ch[i]);
                        ret.push('_');
                    }
                    Some(_) | None => ret.push(ch[i]),
                };
            }
            ret
        };
        dbg!(&chars);

        use std::cmp::max;
        let mut len = 0;
        for i in 0..chars.len() {
            let mut left = i;
            let mut right = i;
            loop {}
            len = max(len, right - left + 1);
        }
        "".to_string()
    }
}

fn main() {
    dbg!(Solution::expand_longest_palindrome(
        "bbbbbdaaaaaa".to_string()
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let tests = [
            ("babad", "aba"),
            ("cbbd", "bb"),
            ("bbbbdaaaaaa", "aaaaaa"),
            ("", ""),
        ];

        for test in tests.iter() {
            let input = test.0.to_string();
            let expected = test.1.to_string();
            assert_eq!(Solution::longest_palindrome(input), expected);
        }
    }

    #[test]
    fn test_expand_longest_palindrome() {
        let tests = [
            ("babad", "bab"),
            ("cbbd", "bb"),
            ("bbbbdaaaaaa", "aaaaaa"),
            ("", ""),
        ];

        for test in tests.iter() {
            let input = test.0.to_string();
            let expected = test.1.to_string();
            assert_eq!(Solution::expand_longest_palindrome(input), expected);
        }
    }
}
