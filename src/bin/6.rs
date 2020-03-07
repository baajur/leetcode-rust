struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let chars: Vec<_> = s.chars().collect();
        let room_in_block = 2 * (num_rows - 1);
        let n_blocks = chars.len() as i32 / room_in_block;

        let mut ans = String::new();
        for r in 0..(num_rows as usize) {
            let mut cur_r = r;
            while let Some(&c) = chars.get(cur_r) {
                ans.push(c);
                if r == 0 || r == (num_rows as usize) - 1 {
                    cur_r += room_in_block as usize;
                } else {
                    if let Some(&cn) = chars.get((cur_r + (room_in_block as usize) - 2 * r)) {
                        ans.push(cn);
                    }
                    cur_r += room_in_block as usize;
                }
            }
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
    fn test_convert() {
        let test_cases = [
            ("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR"),
            ("PAYPALISHIRING", 4, "PINALSIGYAHRPI"),
            ("PAYPALISHIRING", 5, "PHASIYIRPLIGAN"),
            ("", 1, ""),
            ("ABC", 1, "ABC"),
            ("ABC", 2, "ACB"),
        ];

        for &(arg1, arg2, expected) in test_cases.iter() {
            assert_eq!(
                Solution::convert(arg1.to_string(), arg2),
                expected.to_string()
            );
        }
    }
}
