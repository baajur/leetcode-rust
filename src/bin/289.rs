struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }
        let cur_board = board.clone();
        for h in 0..board.len() {
            for w in 0..board[0].len() {
                let live_neighbors = count_live_neighbors(&cur_board, h, w);
                if cur_board[h][w] == 1 {
                    match live_neighbors {
                        0 | 1 => board[h][w] = 0,
                        2 | 3 => (),
                        _ => board[h][w] = 0,
                    };
                } else {
                    match live_neighbors {
                        3 => board[h][w] = 1,
                        _ => (),
                    };
                }
            }
        }
    }
}

fn count_live_neighbors(board: &Vec<Vec<i32>>, h: usize, w: usize) -> i32 {
    // 123
    // 4*5
    // 678
    let max_h = board.len() - 1;
    let max_w = board[0].len() - 1;
    let mut cnt = 0;
    // 1
    if h >= 1 && w >= 1 && board[h - 1][w - 1] == 1 {
        cnt += 1
    }
    // 2
    if h >= 1 && board[h - 1][w] == 1 {
        cnt += 1;
    }
    // 3
    if h >= 1 && w + 1 <= max_w && board[h - 1][w + 1] == 1 {
        cnt += 1;
    }
    // 4
    if w >= 1 && board[h][w - 1] == 1 {
        cnt += 1;
    }
    // 5
    if w + 1 <= max_w && board[h][w + 1] == 1 {
        cnt += 1;
    }
    // 6
    if h + 1 <= max_h && w >= 1 && board[h + 1][w - 1] == 1 {
        cnt += 1
    }
    // 7
    if h + 1 <= max_h && board[h + 1][w] == 1 {
        cnt += 1
    }
    // 8
    if h + 1 <= max_h && w + 1 <= max_w && board[h + 1][w + 1] == 1 {
        cnt += 1
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_of_life() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut board);
        assert_eq!(
            &board,
            &vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]
        );
        Solution::game_of_life(&mut board);
        assert_eq!(
            &board,
            &vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 0, 1], vec![0, 1, 1]]
        );
    }
}
