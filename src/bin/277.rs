struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let cnts = strs
            .into_iter()
            .map(|s| {
                let mut tmp = (0, 0);
                for c in s.chars() {
                    if c == '0' {
                        tmp.0 += 1;
                    } else {
                        tmp.1 += 1;
                    }
                }
                tmp
            })
            .collect::<Vec<_>>();
        let mut dp = vec![vec![vec![0; n as usize + 4]; m as usize + 4]; cnts.len() + 2];
        for i in 0..cnts.len() {
            let (zero, one) = cnts[i];
            for mm in 0..=(m as usize) {
                for nn in 0..=(n as usize) {
                    if mm >= zero && nn >= one {
                        dp[i + 1][mm][nn] =
                            std::cmp::max(dp[i][mm][nn], dp[i][mm - zero][nn - one] + 1);
                    } else {
                        dp[i + 1][mm][nn] = dp[i][mm][nn];
                    }
                }
            }
        }
        dp[cnts.len()][m as usize][n as usize]
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_form() {
        assert_eq!(
            Solution::find_max_form(
                vec![
                    "10".to_string(),
                    "0001".to_string(),
                    "111001".to_string(),
                    "1".to_string(),
                    "0".to_string()
                ],
                5,
                3
            ),
            4
        );
        assert_eq!(
            Solution::find_max_form(
                vec!["10".to_string(), "0".to_string(), "1".to_string(),],
                1,
                1
            ),
            2
        );
        assert_eq!(
            Solution::find_max_form(
                vec![
                    "10".to_string(),
                    "0001".to_string(),
                    "111001".to_string(),
                    "1".to_string(),
                    "0".to_string(),
                ],
                3,
                4
            ),
            3
        );
    }
}
