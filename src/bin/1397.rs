struct Solution;

impl Solution {
    // TODO: wrong answer
    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        let MOD = 1_000_000_007;
        let n = n as usize;
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let evil: Vec<char> = evil.chars().collect();
        let mut dp = vec![vec![vec![vec![0; 2]; 2]; evil.len() + 3]; n as usize + 3];
        dp[0][0][0][0] = 1;
        for i in 0..n {
            for j in 0..evil.len() {
                // i桁目まででs1より大きく、かつs2より小さいことが確定しているなら、i+1桁目は何でも良い
                for k in 0..26 {
                    let c = ('a' as u8 + k as u8) as char;
                    if c == evil[j] {
                        dp[i + 1][j + 1][1][1] += dp[i][j][1][1];
                        dp[i + 1][j + 1][1][1] %= MOD;
                    } else {
                        dp[i + 1][0][1][1] += dp[i][j][1][1];
                        dp[i + 1][0][1][1] %= MOD;
                    }
                }

                // i桁目までs1と同じ、s2より小さいことは確定していて、i+1桁目でs1より大きい文字あるいは同じ文字のとき
                for k in 0..26 {
                    let c = ('a' as u8 + k as u8) as char;
                    if c < s1[i] {
                        continue;
                    }
                    if c == s1[i] {
                        if c == evil[j] {
                            dp[i + 1][j + 1][0][1] += dp[i][j][0][1];
                            dp[i + 1][j + 1][0][1] %= MOD;
                        } else {
                            dp[i + 1][0][0][1] += dp[i][j][0][1];
                            dp[i + 1][0][0][1] %= MOD;
                        }
                    }
                    if c > s1[i] {
                        if c == evil[j] {
                            dp[i + 1][j + 1][1][1] += dp[i][j][0][1];
                            dp[i + 1][j + 1][1][1] %= MOD;
                        } else {
                            dp[i + 1][0][1][1] += dp[i][j][0][1];
                            dp[i + 1][0][1][1] %= MOD;
                        }
                    }
                }

                // i桁目までs2と同じ、s1より大きいことは確定していて、i+1桁目でs2より小さい文字あるいは同じ文字のとき
                for k in 0..26 {
                    let c = ('a' as u8 + k as u8) as char;
                    if c > s2[i] {
                        break;
                    }
                    if c == s2[i] {
                        if c == evil[j] {
                            dp[i + 1][j + 1][1][0] += dp[i][j][1][0];
                            dp[i + 1][j + 1][1][0] %= MOD;
                        } else {
                            dp[i + 1][0][1][0] += dp[i][j][1][0];
                            dp[i + 1][0][1][0] %= MOD;
                        }
                    }
                    if c < s2[i] {
                        if c == evil[j] {
                            dp[i + 1][j + 1][1][1] += dp[i][j][1][0];
                            dp[i + 1][j + 1][1][1] %= MOD;
                        } else {
                            dp[i + 1][0][1][1] += dp[i][j][1][0];
                            dp[i + 1][0][1][1] %= MOD;
                        }
                    }
                }

                // i桁目までs1, s2と同じである場合
                for k in 0..26 {
                    let c = ('a' as u8 + k as u8) as char;
                    if c > s2[i] || c < s1[i] {
                        continue;
                    }
                    if s1[i] != s2[i] {
                        if c == s2[i] {
                            // upper bound
                            if c == evil[j] {
                                dp[i + 1][j + 1][1][0] += dp[i][j][0][0];
                                dp[i + 1][j + 1][1][0] %= MOD;
                            } else {
                                dp[i + 1][0][1][0] += dp[i][j][0][0];
                                dp[i + 1][0][1][0] %= MOD;
                            }
                        }
                        if s1[i] < c && c < s2[i] {
                            // between
                            if c == evil[j] {
                                dp[i + 1][j + 1][1][1] += dp[i][j][0][0];
                                dp[i + 1][j + 1][1][1] %= MOD;
                            } else {
                                dp[i + 1][0][1][1] += dp[i][j][0][0];
                                dp[i + 1][0][1][1] %= MOD;
                            }
                        }
                        if c == s1[i] {
                            // lower bound
                            if c == evil[j] {
                                dp[i + 1][j + 1][0][1] += dp[i][j][0][0];
                                dp[i + 1][j + 1][0][1] %= MOD;
                            } else {
                                dp[i + 1][0][0][1] += dp[i][j][0][0];
                                dp[i + 1][0][0][1] %= MOD;
                            }
                        }
                    } else {
                        if c == s1[i] {
                            if c == evil[j] {
                                dp[i + 1][j + 1][0][0] += dp[i][j][0][0];
                                dp[i + 1][j + 1][0][0] %= MOD;
                            } else {
                                dp[i + 1][0][0][0] += dp[i][j][0][0];
                                dp[i + 1][0][0][0] %= MOD;
                            }
                        }
                    }
                }
            }
        }

        let mut ans = 0;
        for i in 0..evil.len() {
            ans += dp[n][i][0][0];
            ans += dp[n][i][1][1];
            ans += dp[n][i][0][1];
            ans += dp[n][i][1][0];
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
    fn test_find_good_strings() {
        assert_eq!(
            Solution::find_good_strings(3, "aaa".to_string(), "aao".to_string(), "aa".to_string()),
            0
        );
        assert_eq!(
            Solution::find_good_strings(2, "aa".to_string(), "da".to_string(), "b".to_string()),
            51
        );
        assert_eq!(
            Solution::find_good_strings(
                8,
                "leetcode".to_string(),
                "leetgoes".to_string(),
                "leet".to_string()
            ),
            0
        );
        assert_eq!(
            Solution::find_good_strings(2, "gx".to_string(), "gz".to_string(), "x".to_string()),
            2
        );
    }
}
