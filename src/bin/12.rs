struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut digits = vec![0; 4];
        let mut num = num;
        let mut idx = 4;
        while num > 0 {
            digits[idx - 1] = num % 10;
            num /= 10;
            idx -= 1;
        }

        let mut s = String::new();
        for i in 0..4 {
            match digits[i] {
                0 => continue,
                4 => match 3 - i {
                    2 => s.push_str("CD"),
                    1 => s.push_str("XL"),
                    0 => s.push_str("IV"),
                    _ => unreachable!(),
                },
                9 => match 3 - i {
                    2 => s.push_str("CM"),
                    1 => s.push_str("XC"),
                    0 => s.push_str("IX"),
                    _ => unreachable!(),
                },
                x @ 1...3 => match 3 - i {
                    3 => (0..x).for_each(|_| s.push_str("M")),
                    2 => (0..x).for_each(|_| s.push_str("C")),
                    1 => (0..x).for_each(|_| s.push_str("X")),
                    0 => (0..x).for_each(|_| s.push_str("I")),
                    _ => unreachable!(),
                },
                x @ 5...8 => match 3 - i {
                    2 => {
                        s.push('D');
                        (0..(x - 5)).for_each(|_| s.push_str("C"))
                    }
                    1 => {
                        s.push('L');
                        (0..(x - 5)).for_each(|_| s.push_str("X"));
                    }
                    0 => {
                        s.push('V');
                        (0..(x - 5)).for_each(|_| s.push_str("I"));
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        s
    }
}

fn main() {
    dbg!(Solution::int_to_roman(3));
    dbg!(Solution::int_to_roman(1994));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(8), "VIII");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
