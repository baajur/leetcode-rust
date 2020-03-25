struct Solution;

// Though this code works on local machine, I get Runtime Error when submitting this code... :(
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let s: Vec<char> = word.chars().collect();
        dfs(&board, &s, 0, 0)
    }
}

fn dfs(board: &Vec<Vec<char>>, s: &[char], h: usize, w: usize) -> bool {
    if s.is_empty() {
        return true;
    }

    if s[0] != board[h][w] {
        return false;
    }

    let max_h = board.len();
    let max_w = board[0].len();

    // up
    if h >= 1 {
        if dfs(board, &s[1..], h - 1, w) {
            return true;
        }
    }
    // down
    if h + 1 <= max_h {
        if dfs(board, &s[1..], h + 1, w) {
            return true;
        }
    }
    // left
    if w >= 1 {
        if dfs(board, &s[1..], h, w - 1) {
            return true;
        }
    }
    // right
    if w + 1 <= max_w {
        if dfs(board, &s[1..], h, w + 1) {
            return true;
        }
    }

    false
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_exist() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(Solution::exist(board.clone(), "ABCCED".to_string()));
        assert!(Solution::exist(board.clone(), "SEE".to_string()));
        assert!(Solution::exist(board.clone(), "SFDEESCEBA".to_string()));
        assert!(!Solution::exist(board.clone(), "ABCB".to_string()));

        let board = vec![vec!['A']];
        assert!(Solution::exist(board.clone(), "A".to_string()));
        assert!(!Solution::exist(board.clone(), "AA".to_string()));
        assert!(!Solution::exist(board.clone(), "".to_string()));

        let mut board = Vec::with_capacity(200);
        for _ in 0..200 {
            board.push(vec!['A'; 200]);
        }
        assert!(Solution::exist(board.clone(), "A".repeat(1000)));
    }
}
