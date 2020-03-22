struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        if n < 4 || n > 12 {
            return vec![];
        }
        let mut ans = Vec::new();
        for i in 1..(n - 2) {
            for j in (i + 1)..(n - 1) {
                for k in (j + 1)..n {
                    let ip = s[..i].iter().collect::<String>()
                        + "."
                        + &s[i..j].iter().collect::<String>().as_str()
                        + "."
                        + &s[j..k].iter().collect::<String>().as_str()
                        + "."
                        + &s[k..].iter().collect::<String>().as_str();
                    if is_valid(&ip) {
                        ans.push(ip);
                    }
                }
            }
        }
        ans
    }
}

fn is_valid(s: &str) -> bool {
    s.split(".")
        .map(|v| {
            if v.len() >= 2 && v.starts_with("0") {
                -1
            } else {
                v.parse::<i32>().unwrap_or(-1)
            }
        })
        .all(|v| 0 <= v && v <= 255)
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_ip_addresses() {
        assert_eq!(
            Solution::restore_ip_addresses("25525511135".to_string()),
            vec!["255.255.11.135", "255.255.111.35"]
        );
        assert_eq!(
            Solution::restore_ip_addresses("1111".to_string()),
            vec!["1.1.1.1"]
        );
        assert_eq!(
            Solution::restore_ip_addresses("111111".to_string()),
            vec![
                "1.1.1.111",
                "1.1.11.11",
                "1.1.111.1",
                "1.11.1.11",
                "1.11.11.1",
                "1.111.1.1",
                "11.1.1.11",
                "11.1.11.1",
                "11.11.1.1",
                "111.1.1.1"
            ]
        );
        assert_eq!(
            Solution::restore_ip_addresses("010010".to_string()),
            vec!["0.10.0.10", "0.100.1.0",]
        );
        assert!(Solution::restore_ip_addresses("".to_string()).is_empty());
        assert!(Solution::restore_ip_addresses("1".to_string()).is_empty());
        assert!(Solution::restore_ip_addresses("11".to_string()).is_empty());
        assert!(Solution::restore_ip_addresses("111".to_string()).is_empty());
        assert_eq!(
            Solution::restore_ip_addresses("1111".to_string()),
            vec!["1.1.1.1"]
        );
        assert_eq!(
            Solution::restore_ip_addresses("123123123123".to_string()),
            vec!["123.123.123.123"]
        );
        assert!(Solution::restore_ip_addresses("1231231231234".to_string()).is_empty());
        assert!(Solution::restore_ip_addresses("111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string()).is_empty());
    }
}
