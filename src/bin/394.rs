struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut ans = String::new();
        let s: Vec<char> = s.chars().collect();
        solve(&mut ans, &s);
        ans
    }
}

fn solve(ans: &mut String, substr: &[char]) {
    let mut iter = substr.into_iter();
    let mut index = 0;
    while index < substr.len() {
        let c = substr[index];
        if !c.is_ascii_digit() && c != '[' {
            ans.push(c);
            index += 1;
            continue;
        }
        if c.is_ascii_digit() {
            let mut repeat = (c as u32 - '0' as u32) as usize;
            while substr[index + 1].is_ascii_digit() {
                let next_d = (substr[index + 1] as u32 - '0' as u32) as usize;
                repeat = repeat * 10 + next_d;
                index += 1;
            }
            index += 2; // proceed to x, such that 3[x
            let mut brackets = 1;
            let start_index = index;
            let mut end_index = index;
            loop {
                match substr[index] {
                    ']' => brackets -= 1,
                    '[' => brackets += 1,
                    _ => (),
                };
                if brackets == 0 {
                    end_index = index;
                    index += 1;
                    break;
                }
                index += 1;
            }
            for _ in 0..repeat {
                solve(ans, &substr[start_index..end_index]);
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
    fn decode_string() {
        assert_eq!(
            Solution::decode_string("3[a]2[bc]".to_string()),
            "aaabcbc".to_string()
        );
        assert_eq!(
            Solution::decode_string("3[a2[c]]".to_string()),
            "accaccacc".to_string()
        );
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".to_string()),
            "abcabccdcdcdef".to_string()
        );
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".to_string()),
            "abcabccdcdcdef".to_string()
        );
        assert_eq!(
            Solution::decode_string("100[leetcode]".to_string()),
            (0..100).into_iter().fold(String::new(), |mut acc, _| {
                acc.push_str("leetcode");
                acc
            }),
        );
        assert_eq!(
            Solution::decode_string("abc".to_string()),
            "abc".to_string()
        );
    }
}
