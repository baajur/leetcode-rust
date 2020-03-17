struct Solution;

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        if n == 2 {
            return k - 1;
        }
        solve(n, k)
    }
}

fn solve(row: i32, col: i32) -> i32 {
    match col {
        1 => 0,
        x if x == 2i32.pow(row as u32 - 1) => {
            if row % 2 == 0 {
                1
            } else {
                0
            }
        }
        x => {
            let parent = solve(row - 1, (col + 1) / 2);
            if parent == 0 && col % 2 != 0 || parent == 1 && col % 2 == 0 {
                0
            } else {
                1
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
    fn test_kth_grammar() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
        assert_eq!(Solution::kth_grammar(3, 1), 0);
        assert_eq!(Solution::kth_grammar(3, 4), 0);
        assert_eq!(Solution::kth_grammar(4, 1), 0);
        assert_eq!(Solution::kth_grammar(4, 2), 1);
        assert_eq!(Solution::kth_grammar(4, 3), 1);
        assert_eq!(Solution::kth_grammar(4, 4), 0);
        assert_eq!(Solution::kth_grammar(4, 5), 1);
        assert_eq!(Solution::kth_grammar(5, 10), 0);
        assert_eq!(Solution::kth_grammar(5, 11), 0);
        assert_eq!(Solution::kth_grammar(5, 12), 1);
    }
}
