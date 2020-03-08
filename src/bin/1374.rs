struct Solution;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n % 2 != 0 {
            (0..n).map(|_| 'a').collect()
        } else {
            let mut s: String = (0..(n - 1)).map(|_| 'a').collect();
            s.push('b');
            s
        }
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_the_string() {
        assert_eq!(Solution::generate_the_string(2), "ab");
        assert_eq!(Solution::generate_the_string(3), "aaa");
        assert_eq!(Solution::generate_the_string(7), "aaaaaaa");
        assert_eq!(Solution::generate_the_string(8), "aaaaaaab");
    }
}
