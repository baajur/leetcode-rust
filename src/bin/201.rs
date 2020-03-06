struct Solution;

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut ans = 0;
        let mut m = m;
        let mut n = n;

        for i in 0..32 {
            let mask = 1 << i;
            if n == m && (n & 1 != 0) {
                ans += mask;
            }
            n = n >> 1;
            m = m >> 1;
        }

        ans
    }

    pub fn naive_range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut ans = m;
        for x in m..=n {
            ans = ans & x;
        }
        ans
    }
}

fn main() {
    ()
}

mod tests {
    use super::*;

    #[test]
    fn test_range_bitwise_and() {
        let test_cases = [(5, 7, 4), (0, 1, 0)];
        for test in test_cases.iter() {
            assert_eq!(Solution::range_bitwise_and(test.0, test.1), test.2);
        }
    }
}
