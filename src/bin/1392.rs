struct Solution;

impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();

        let radix = 100_000_007;
        let mut ans = 0;
        let mut head_hash = 0u64;
        let mut tail_hash = 0u64;
        let mut tail_r = 1;
        for i in 0..(s.len() - 1) {
            head_hash = head_hash.wrapping_mul(radix).wrapping_add(s[i] as u64);
            tail_hash = tail_hash.wrapping_add((s[s.len() - 1 - i] as u64).wrapping_mul(tail_r));
            tail_r = tail_r.wrapping_mul(radix);
            if head_hash == tail_hash {
                ans = i + 1;
            }
        }
        s.drain(..ans).collect()
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_prefix() {
        assert_eq!(Solution::longest_prefix("level".to_string()), "l");
        assert_eq!(Solution::longest_prefix("ababab".to_string()), "abab");
        assert_eq!(Solution::longest_prefix("leetcodeleet".to_string()), "leet");
        assert_eq!(Solution::longest_prefix("a".to_string()), "");
    }
}
