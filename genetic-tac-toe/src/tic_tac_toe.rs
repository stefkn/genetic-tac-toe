/// Represents a Tic Tac Toe game.
pub struct TicTacToe {
    board: [char; 9],
    pub current_winner: Option<char>,
}

impl TicTacToe {
    /// Creates a new instance of TicTacToe.
    pub fn new() -> Self {
        Self {
            board: [' '; 9],
            current_winner: None,
        }
    }

    // Returns the current state of the board.
    pub fn get_board(&self) -> [char; 9] {
        self.board
    }

    /// Prints the current state of the Tic Tac Toe board.
    pub fn print_board(&self) {
        for row in self.board.chunks(3) {
            println!("| {} | {} | {} |", row[0], row[1], row[2]);
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

    /// Checks if a player has won the game.
    ///
    /// # Arguments
    ///
    /// * `square` - The index of the last move made.
    /// * `letter` - The letter of the player who made the last move.
    ///
    /// # Returns
    ///
    /// Returns `true` if the player has won, `false` otherwise.
    pub fn winner(&self, square: usize, letter: char) -> bool {
        let row_index = square / 3;
        let col_index = square % 3;

        // Check row
        if self.board[row_index * 3..row_index * 3 + 3].iter().all(|&x| x == letter) {
            return true;
        }
        // Check column
        if (0..3).all(|i| self.board[col_index + i * 3] == letter) {
            return true;
        }
        // Check diagonals
        if square % 2 == 0 {
            if (0..3).all(|i| self.board[i * 4] == letter) || (0..3).all(|i| self.board[i * 2 + 2] == letter) {
                return true;
            }
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
    fn test_winner() {
        let mut game = TicTacToe::new();
        game.make_move(0, 'X');
        game.make_move(1, 'X');
        game.make_move(2, 'X');
        assert!(game.winner(2, 'X'));
    }

    #[test]
    fn test_available_moves() {
        let mut game = TicTacToe::new();
        game.make_move(0, 'X');
        let available_moves = game.available_moves();
        assert_eq!(available_moves.len(), 8);
        assert!(!available_moves.contains(&0));
    }

    #[test]
    fn test_get_board() {
        let game = TicTacToe::new();
        assert_eq!(game.get_board(), [' '; 9]);
    }
}