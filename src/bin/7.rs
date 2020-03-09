struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == std::i32::MIN || x == 0 {
            return 0;
        }

        let mut ret = 0i32;
        let is_negative = x < 0;
        let mut x = if is_negative { -1 * x } else { x };
        while x != 0 {
            ret = match ret.checked_mul(10) {
                Some(x) => x,
                None => return 0,
            };
            ret = match ret.checked_add(x % 10) {
                Some(x) => x,
                None => return 0,
            };
            x /= 10;
        }

        if is_negative {
            -1 * ret
        } else {
            ret
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
    fn test_reverse() {
        let tests = [
            (0, 0),
            (std::i32::MIN, 0),
            (std::i32::MAX, 0), // the reverse of 2147483647 is 7463847412 (overflow)
            (-123, -321),
            (120, 21),
            (1200, 21),
            (1, 1),
        ];
        for test in tests.iter() {
            assert_eq!(Solution::reverse(test.0), test.1);
        }
    }
}
