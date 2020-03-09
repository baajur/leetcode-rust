struct Solution;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let int_ch = str
            .chars()
            .skip_while(|c| c.is_ascii_whitespace())
            .take_while(|c| c == &'-' || c == &'+' || c.is_ascii_digit())
            .collect::<Vec<_>>();
        if int_ch.is_empty() {
            return 0;
        }

        #[derive(PartialEq)]
        enum Sign {
            Plus,
            Minus,
            None,
        };

        let sign = match int_ch[0] {
            '-' => Sign::Minus,
            '+' => Sign::Plus,
            _ => Sign::None,
        };

        let digits_begin_from = if sign == Sign::None { 0 } else { 1 };
        let digits = int_ch
            .get(digits_begin_from..)
            .unwrap()
            .iter()
            .take_while(|c| c.is_ascii_digit())
            .map(|c| (*c as u8 - '0' as u8) as i32)
            .collect::<Vec<_>>();

        let mut int = 0i32;
        for d in digits {
            int = match int.checked_mul(10) {
                Some(x) => x,
                None => match sign {
                    Sign::Minus => return std::i32::MIN,
                    _ => return std::i32::MAX,
                },
            };
            int = match int.checked_add(d) {
                Some(x) => x,
                None => match sign {
                    Sign::Minus => return std::i32::MIN,
                    _ => return std::i32::MAX,
                },
            };
        }

        match sign {
            Sign::Minus => -1 * int,
            _ => int,
        }
    }
}

fn main() {
    dbg!(Solution::my_atoi("    -43foobar".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi() {
        let tests = [
            ("     -42", -42),
            ("4193 with words", 4193),
            ("words and 987", 0),
            ("-91283472332", -2147483648),
            ("91283472332", 2147483647),
            ("   -3+23552 foo", -3),
            ("   -+ foo", 0),
            ("  3-", 3),
            ("   -with", 0),
            ("", 0),
        ];

        for test in tests.iter() {
            let input = test.0.to_string();
            let expected = test.1;
            assert_eq!(Solution::my_atoi(input), expected);
        }
    }
}
