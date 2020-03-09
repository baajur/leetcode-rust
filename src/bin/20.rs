struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        dbg!(&s);
        let ch: Vec<_> = s.chars().collect();
        if ch.is_empty() {
            return true;
        }
        if ch.len() % 2 != 0 {
            return false;
        }

        let mut left_par = Vec::new();
        for i in 0..ch.len() {
            if left_par.is_empty() {
                if is_close(ch[i]) {
                    return false;
                } else {
                    left_par.push(ch[i]);
                }
            } else {
                if is_close(ch[i]) {
                    let l = left_par.pop().unwrap();
                    if close(l) != ch[i] {
                        return false;
                    }
                } else {
                    left_par.push(ch[i]);
                }
            }
        }

        left_par.is_empty()
    }
}

fn is_close(c: char) -> bool {
    c == ')' || c == '}' || c == ']'
}

fn is_open(c: char) -> bool {
    c == '(' || c == '{' || c == '{'
}

fn close(b: char) -> char {
    match b {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        _ => unreachable!(),
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let tests = [
            ("(())", true),
            ("[]", true),
            ("({])", false),
            ("", true),
            ("(", false),
            ("()[]{}", true),
            ("{[}]", false),
        ];

        for test in tests.iter() {
            assert_eq!(Solution::is_valid(test.0.to_string()), test.1);
        }
    }
}
