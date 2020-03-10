struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = 1;
        for i in 0..(m as usize) {
            for j in 0..(n as usize) {
                if i + 1 < m as usize {
                    // go right
                    dp[i + 1][j] += dp[i][j];
                }
                if j + 1 < n as usize {
                    // go down
                    dp[i][j + 1] += dp[i][j];
                }
            }
        }
        dp[m as usize - 1][n as usize - 1]
    }

    pub fn math_unique_paths(m: i32, n: i32) -> i32 {
        // to avoid overflow, we cannot use straightforward way of factorial function
        // EDIT: Still overflow happens by the following code!!!!
        let (m, n) = if m >= n { (m, n) } else { (n, m) };
        let mut ret = 1;
        let mut cur_n = 1;
        for i in m..(m + n - 1) {
            ret *= i;
            if cur_n <= n - 1 {
                ret /= cur_n;
                cur_n += 1;
            }
        }
        ret
    }

    fn factorial(n: i32) -> i32 {
        match n {
            0 | 1 => 1,
            _ => n * Solution::factorial(n - 1),
        }
    }
}

fn main() {
    dbg!(Solution::unique_paths(7, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(2, 3), 3);
        assert_eq!(Solution::unique_paths(1, 1), 1);
        assert_eq!(Solution::unique_paths(1, 3), 1);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(10, 10), 48620);
        assert_eq!(Solution::unique_paths(51, 9), 1916797311);
    }

    #[test]
    fn test_math_unique_paths() {
        assert_eq!(Solution::math_unique_paths(3, 2), 3);
        assert_eq!(Solution::math_unique_paths(2, 3), 3);
        assert_eq!(Solution::math_unique_paths(1, 1), 1);
        assert_eq!(Solution::math_unique_paths(1, 3), 1);
        assert_eq!(Solution::math_unique_paths(7, 3), 28);
        assert_eq!(Solution::math_unique_paths(10, 10), 48620);
        // assert_eq!(Solution::unique_paths(51, 9), 1916797311); // overflow
    }
}
