struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        solve(&mut ans, 0, n, 0, "".to_string(), n);
        ans
    }
}

fn solve(
    ans: &mut Vec<String>,
    opening: i32,
    rest_left: i32,
    pos: i32,
    cur_str: String,
    n_pairs: i32,
) {
    if pos == n_pairs * 2 {
        ans.push(cur_str);
        return;
    }

    if opening > 0 {
        // set right parenthesis
        let mut s = cur_str.clone();
        s.push(')');
        solve(ans, opening - 1, rest_left, pos + 1, s, n_pairs);
    }
    if rest_left > 0 {
        // set left parenthesis
        let mut s = cur_str.clone();
        s.push('(');
        solve(ans, opening + 1, rest_left - 1, pos + 1, s, n_pairs);
    }
}

fn main() {
    dbg!(Solution::generate_parenthesis(3));
    dbg!(Solution::generate_parenthesis(8));
    dbg!(Solution::generate_parenthesis(10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        ()
    }
}
