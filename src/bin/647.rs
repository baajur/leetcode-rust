struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut cnt = 0;
        for i in 0..s.len() {
            if 0 < i {
                {
                    let mut left = i - 1;
                    let mut right = i;
                    while s[left] == s[right] {
                        cnt += 1;
                        if left == 0 || right == s.len() - 1 {
                            break;
                        } else {
                            left -= 1;
                            right += 1;
                        }
                    }
                }
                if i < s.len() - 1 {
                    cnt += 1;
                    let mut left = i - 1;
                    let mut right = i + 1;
                    while s[left] == s[right] {
                        cnt += 1;
                        if left == 0 || right == s.len() - 1 {
                            break;
                        } else {
                            left -= 1;
                            right += 1;
                        }
                    }
                } else {
                    cnt += 1;
                }
            } else {
                cnt += 1;
            }
        }
        cnt
    }

    pub fn simple_count_substrings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut ans = 0;
        for center in 0..(2 * s.len() - 1) {
            if center % 2 == 0 {
                let c = center / 2;
                ans += 1;
                let mut left = c.checked_sub(1);
                let mut right = match c + 1 {
                    r if r >= s.len() => None,
                    r => Some(r),
                };
                while left.is_some() && right.is_some() && s[left.unwrap()] == s[right.unwrap()] {
                    ans += 1;
                    left = left.unwrap().checked_sub(1);
                    right = match right.unwrap() + 1 {
                        r if r >= s.len() => None,
                        r => Some(r),
                    };
                }
            } else {
                let c = (center + 1) / 2;
                let mut left = c.checked_sub(1);
                let mut right = Some(c);
                dbg!(&c, &left, &right);
                while left.is_some() && right.is_some() && s[left.unwrap()] == s[right.unwrap()] {
                    ans += 1;
                    left = left.unwrap().checked_sub(1);
                    right = match right.unwrap() + 1 {
                        r if r >= s.len() => None,
                        r => Some(r),
                    };
                }
            }
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
    fn count_substrings() {
        assert_eq!(Solution::count_substrings("abc".to_string()), 3);
        assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
        assert_eq!(Solution::count_substrings("aa".to_string()), 3);
        assert_eq!(Solution::count_substrings("abba".to_string()), 6);
        assert_eq!(Solution::count_substrings("abcba".to_string()), 7);
    }

    #[test]
    fn simple_count_substrings() {
        assert_eq!(Solution::simple_count_substrings("abc".to_string()), 3);
        assert_eq!(Solution::simple_count_substrings("aaa".to_string()), 6);
        assert_eq!(Solution::simple_count_substrings("aa".to_string()), 3);
        assert_eq!(Solution::simple_count_substrings("abba".to_string()), 6);
        assert_eq!(Solution::simple_count_substrings("abcba".to_string()), 7);
    }
}
