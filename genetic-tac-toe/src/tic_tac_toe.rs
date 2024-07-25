/// Represents a Tic Tac Toe game.
pub struct TicTacToe {
    board: [char; 9],
    current_winner: Option<char>,
}

impl TicTacToe {
    /// Creates a new instance of TicTacToe.
    pub fn new() -> Self {
        Self {
            board: [' '; 9],
            current_winner: None,
        }
    }


    /// Returns a vector of available moves on the board.
    pub fn available_moves(&self) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x == ' ' { Some(i) } else { None })
            .collect()
    }

    /// Makes a move on the board.
    ///
    /// # Arguments
    ///
    /// * `square` - The index of the square to make the move on.
    /// * `letter` - The letter to place on the square.
    ///
    /// # Returns
    ///
    /// Returns `true` if the move was successful, `false` otherwise.
    pub fn make_move(&mut self, square: usize, letter: char) -> bool {
        if self.board[square] == ' ' {
            self.board[square] = letter;
            if self.winner(square, letter) {
                self.current_winner = Some(letter);
            }
            return true;
        }
        false
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = TicTacToe::new();
        assert_eq!(game.board, [' '; 9]);
        assert_eq!(game.current_winner, None);
    }

    #[test]
    fn test_make_move() {
        let mut game = TicTacToe::new();
        assert!(game.make_move(0, 'X'));
        assert_eq!(game.board[0], 'X');
        assert!(!game.make_move(0, 'O')); // Square already taken
    }


    #[test]
    fn test_available_moves() {
        let mut game = TicTacToe::new();
        game.make_move(0, 'X');
        let available_moves = game.available_moves();
        assert_eq!(available_moves.len(), 8);
        assert!(!available_moves.contains(&0));
    }
}