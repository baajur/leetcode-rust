#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    None,
    Circle, // player 1
    Cross,  // player 2
}

struct TicTacToe {
    size: usize,
    board: Vec<Vec<State>>,
    winner: Option<i32>,
}

impl TicTacToe {
    fn new(n: i32) -> Self {
        Self {
            size: n as usize,
            board: vec![vec![State::None; n as usize]; n as usize],
            winner: None,
        }
    }

    fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        if self.winner.is_some() {
            return self.winner.unwrap();
        }

        self.board[row as usize][col as usize] = if player == 1 {
            State::Circle
        } else {
            State::Cross
        };

        if self.check(row, col, player) {
            self.winner = Some(player);
            player
        } else {
            0
        }
    }

    fn check(&self, row: i32, col: i32, player: i32) -> bool {
        let st = if player == 1 {
            State::Circle
        } else {
            State::Cross
        };
        // check vertivally
        if self.board.iter().all(|row| row[col as usize] == st) {
            return true;
        }

        // check horizontally
        if self.board[row as usize].iter().all(|elem| *elem == st) {
            return true;
        }

        // check diagonally (leftup to rightbottom)
        if row - col == 0 && self.board.iter().enumerate().all(|(i, row)| row[i] == st) {
            return true;
        }

        // leftbottom to rightup
        if row + col == self.board.len() as i32 - 1
            && self
                .board
                .iter()
                .enumerate()
                .all(|(i, row)| row[self.board.len() - 1 - i] == st)
        {
            return true;
        }

        false
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tic_tac_toe() {
        let mut toe = TicTacToe::new(3);
        assert_eq!(toe.make_a_move(0, 0, 1), 0);
        assert_eq!(toe.make_a_move(0, 2, 2), 0);
        assert_eq!(toe.make_a_move(2, 2, 1), 0);
        assert_eq!(toe.make_a_move(1, 1, 2), 0);
        assert_eq!(toe.make_a_move(2, 0, 1), 0);
        assert_eq!(toe.make_a_move(1, 0, 2), 0);
        assert_eq!(toe.make_a_move(2, 1, 1), 1);
        assert_eq!(toe.make_a_move(0, 1, 2), 1);

        let mut toe = TicTacToe::new(3);
        assert_eq!(toe.make_a_move(1, 1, 1), 0);
        assert_eq!(toe.make_a_move(0, 2, 2), 0);
        assert_eq!(toe.make_a_move(2, 2, 1), 0);
        assert_eq!(toe.make_a_move(0, 1, 2), 0);
        assert_eq!(toe.make_a_move(0, 0, 1), 1);
        assert_eq!(toe.make_a_move(1, 0, 2), 1);

        let mut toe = TicTacToe::new(3);
        assert_eq!(toe.make_a_move(1, 1, 1), 0);
        assert_eq!(toe.make_a_move(0, 0, 2), 0);
        assert_eq!(toe.make_a_move(0, 2, 1), 0);
        assert_eq!(toe.make_a_move(0, 1, 2), 0);
        assert_eq!(toe.make_a_move(2, 0, 1), 1);
        assert_eq!(toe.make_a_move(1, 0, 2), 1);

        let mut toe = TicTacToe::new(3);
        assert_eq!(toe.make_a_move(0, 0, 1), 0);
        assert_eq!(toe.make_a_move(1, 0, 2), 0);
        assert_eq!(toe.make_a_move(0, 1, 1), 0);
        assert_eq!(toe.make_a_move(1, 1, 2), 0);
        assert_eq!(toe.make_a_move(0, 2, 1), 1);
        assert_eq!(toe.make_a_move(1, 2, 2), 1);
    }
}
