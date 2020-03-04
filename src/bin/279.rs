struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        use std::cmp::min;

        let mut squares = Vec::new(); // square numbers up to n
        {
            let mut cur = 1;
            while cur * cur <= n {
                squares.push(cur * cur);
                cur += 1;
            }
        }
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 0;

        for i in 1..=(n as usize) {
            let mut min_cnt = 1 << 30;
            let mut iter = squares.iter().take_while(|&&s| i as i32 >= s);
            while let Some(&sq) = iter.next() {
                min_cnt = min(min_cnt, dp[i - sq as usize]);
            }
            dp[i] = min_cnt + 1;
        }

        dp[n as usize]
    }
}

fn main() {
    dbg!(Solution::num_squares(100000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_squares() {
        let test_cases = [
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 1),
            (5, 2),
            (6, 3),
            (7, 4),
            (8, 2),
            (9, 1),
            (10, 2),
            (11, 3),
            (12, 3),
            (13, 2),
        ];

        for &(input, expected) in test_cases.iter() {
            assert_eq!(Solution::num_squares(input), expected);
        }
    }
}
