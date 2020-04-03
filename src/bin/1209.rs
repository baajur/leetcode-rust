struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack = Vec::new();
        for c in s.chars() {
            stack.push(c);
            while stack.len() >= k as usize
                && stack
                    .iter()
                    .rev()
                    .take(k as usize)
                    .all(|s| Some(s) == stack.last())
            {
                for _ in 0..(k as usize) {
                    stack.pop();
                }
            }
        }
        stack.into_iter().collect::<String>()
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(Solution::remove_duplicates("abcd".to_string(), 2), "abcd");
        assert_eq!(
            Solution::remove_duplicates("deeedbbcccbdaa".to_string(), 3),
            "aa"
        );
        assert_eq!(
            Solution::remove_duplicates("pbbcggttciiippooaais".to_string(), 2),
            "ps"
        );
    }
}
