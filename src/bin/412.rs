struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        if n <= 0 {
            return vec![];
        }
        (1..=n)
            .map(|a| {
                if a % 15 == 0 {
                    "FizzBuzz".to_string()
                } else if a % 5 == 0 {
                    "Buzz".to_string()
                } else if a % 3 == 0 {
                    "Fizz".to_string()
                } else {
                    a.to_string()
                }
            })
            .collect()
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(
            Solution::fizz_buzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz",
            ]
        );
        assert_eq!(Solution::fizz_buzz(2), vec!["1", "2"]);
        assert!(Solution::fizz_buzz(0).is_empty());
    }
}
