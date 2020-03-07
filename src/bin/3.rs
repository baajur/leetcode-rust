struct Solution;

impl Solution {
    // two-pointers. complexity: O(N)
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        use std::collections::HashSet;
        let chars: Vec<_> = s.chars().collect();
        let mut set = HashSet::new();
        let mut right = 0;
        let mut ans = 0;
        for left in 0..chars.len() {
            while right < chars.len() && !set.contains(&chars[right]) {
                set.insert(chars[right]);
                right += 1;
            }

            ans = max(ans, right - left);

            if right == left {
                right += 1;
            } else {
                set.remove(&chars[left]);
            }
        }

        ans as i32
    }

    // complexity: O(N^2), where N is the length of s
    pub fn slow_length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        use std::collections::HashSet;
        let chars: Vec<_> = s.chars().collect();
        let mut ans = 0;
        for i in 0..chars.len() {
            let mut set = HashSet::new();
            let count = chars
                .iter()
                .skip(i)
                .take_while(|c| {
                    if set.contains(c) {
                        false
                    } else {
                        set.insert(*c);
                        true
                    }
                })
                .count() as i32;
            ans = max(ans, count);
        }
        ans
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        let test_cases = [
            ("abcabcbb", 3),
            ("bbbbb", 1),
            ("pwwkew", 3),
            ("aaaaabcde", 5),
            ("dvdf", 3),
        ];

        for &(input, expected) in test_cases.iter() {
            assert_eq!(
                Solution::length_of_longest_substring(input.to_string()),
                expected
            );
        }
    }

    #[test]
    fn test_slow_length_of_longest_substring() {
        let test_cases = [
            ("abcabcbb", 3),
            ("bbbbb", 1),
            ("pwwkew", 3),
            ("aaaaabcde", 5),
            ("dvdf", 3),
        ];

        for &(input, expected) in test_cases.iter() {
            assert_eq!(
                Solution::slow_length_of_longest_substring(input.to_string()),
                expected
            );
        }
    }
}
