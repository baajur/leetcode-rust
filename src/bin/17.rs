struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let digits: Vec<usize> = digits
            .chars()
            .map(|c| ((c as u8) - ('0' as u8)) as usize)
            .collect();
        let mappings = vec![
            vec![],                   // 0
            vec![],                   // 1
            vec!['a', 'b', 'c'],      // 2
            vec!['d', 'e', 'f'],      // 3
            vec!['g', 'h', 'i'],      // 4
            vec!['j', 'k', 'l'],      // 5
            vec!['m', 'n', 'o'],      // 6
            vec!['p', 'q', 'r', 's'], // 7
            vec!['t', 'u', 'v'],      // 8
            vec!['w', 'x', 'y', 'z'], // 9
        ];
        dfs(&mappings, vec!["".to_string()], &digits, 0)
    }
}

fn dfs(mappings: &Vec<Vec<char>>, v: Vec<String>, digits: &Vec<usize>, pos: usize) -> Vec<String> {
    if pos >= digits.len() {
        return v;
    }

    let d = digits[pos];

    let mut next_v = Vec::new();
    for s in v.into_iter() {
        for &c in mappings[d].iter() {
            let mut cl = s.clone();
            cl.push(c);
            next_v.push(cl);
        }
    }

    dfs(mappings, next_v, digits, pos + 1)
}

fn main() {
    dbg!(Solution::letter_combinations("23".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations("22".to_string()),
            vec!["aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc"]
        );
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a", "b", "c"]
        );
        assert_eq!(
            Solution::letter_combinations("76".to_string()),
            vec!["pm", "pn", "po", "qm", "qn", "qo", "rm", "rn", "ro", "sm", "sn", "so"]
        );
        assert!(Solution::letter_combinations("".to_string()).is_empty());
    }
}
