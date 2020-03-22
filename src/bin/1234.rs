struct Solution;

impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut ch_counts = vec![0; 4]; // 0: Q, 1: W, 2: E, 3: R
        for i in 0..n {
            ch_counts[char_to_index(s[i])] += 1;
        }

        let mut right = 0;
        use std::cmp::min;
        let mut ans = s.len();

        for left in 0..s.len() {
            if cur_right_ok(&ch_counts, n) {
                ans = min(ans, right - left);
                if left == right {
                    right += 1;
                } else {
                    ch_counts[char_to_index(s[left])] += 1;
                }
            } else {
                while right < n && next_right_ng(&ch_counts, s[right], n) {
                    ch_counts[char_to_index(s[right])] -= 1;
                    right += 1;
                }
                if right < n {
                    ch_counts[char_to_index(s[right])] -= 1;
                    right += 1;
                }
                if cur_right_ok(&ch_counts, n) {
                    ans = min(ans, right - left);
                }

                if left == right {
                    right += 1;
                } else {
                    ch_counts[char_to_index(s[left])] += 1;
                }
            }
        }
        ans as i32
    }
}

fn cur_right_ok(counts: &[i32], len: usize) -> bool {
    counts.iter().all(|&v| v <= len as i32 / 4)
}

fn next_right_ng(counts: &[i32], next_char: char, len: usize) -> bool {
    let upper = len as i32 / 4;
    let idx = char_to_index(next_char);
    counts
        .iter()
        .enumerate()
        .map(|(i, &v)| if i == idx { v - 1 } else { v })
        .any(|v| v > upper)
}

fn char_to_index(c: char) -> usize {
    match c {
        'Q' => 0,
        'W' => 1,
        'E' => 2,
        'R' => 3,
        _ => unreachable!(),
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced_string() {
        assert_eq!(Solution::balanced_string("QWER".to_string()), 0);
        assert_eq!(Solution::balanced_string("QQWE".to_string()), 1);
        assert_eq!(Solution::balanced_string("QQQW".to_string()), 2);
        assert_eq!(Solution::balanced_string("QQQQ".to_string()), 3);
        assert_eq!(Solution::balanced_string("WWQQRRRRQRQQ".to_string()), 4);
    }
}
