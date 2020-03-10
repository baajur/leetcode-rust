struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let is_negative = n < 0;
        let n = if is_negative { -1 * n } else { n };
        let is_odd = n % 2 != 0;
        let mut even_n = if is_odd { n - 1 } else { n };

        let mut res = 1.0;
        let mut powi = x;
        for i in 1..32 {
            powi = powi * powi;
            if even_n & 1 << i != 0 {
                res *= powi;
            }
        }

        if is_odd {
            res *= x;
        }
        if is_negative {
            res = 1.0 / res;
        }

        res
    }

    pub fn recursive_my_pow(x: f64, n: i32) -> f64 {
        if n >= 0 {
            if n % 2 == 0 {
                Solution::my_power(x, n)
            } else {
                x * Solution::my_power(x, n - 1)
            }
        } else {
            let m = -1 * n;
            let denominator = if m % 2 == 0 {
                Solution::my_power(x, m)
            } else {
                x * Solution::my_power(x, m - 1)
            };

            1.0 / denominator
        }
    }

    fn my_power(x: f64, n: i32) -> f64 {
        match n {
            0 => 1.0,
            1 => x,
            m if m % 2 == 0 => {
                let y = Solution::my_power(x, n / 2);
                y * y
            }
            _ => {
                let y = Solution::my_power(x, n / 2);
                x * y * y
            }
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
    fn test_my_pow() {
        let epsilon = 0.000001;
        assert!((Solution::my_pow(2.0, 10) - 1024.0).abs() <= epsilon);
        assert!((Solution::my_pow(2.0, 11) - 2048.0).abs() <= epsilon);
        assert!((Solution::my_pow(2.0, -2) - 0.25).abs() <= epsilon);
        assert!((Solution::my_pow(3.0, 4) - 81.0).abs() <= epsilon);
    }

    #[test]
    fn test_recursive_my_pow() {
        let epsilon = 0.000001;
        assert!((Solution::recursive_my_pow(2.0, 10) - 1024.0).abs() <= epsilon);
        assert!((Solution::recursive_my_pow(2.0, 11) - 2048.0).abs() <= epsilon);
        assert!((Solution::recursive_my_pow(2.0, -2) - 0.25).abs() <= epsilon);
        assert!((Solution::recursive_my_pow(3.0, 4) - 81.0).abs() <= epsilon);
    }
}
