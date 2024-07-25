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
    fn test_available_moves() {
        let mut game = TicTacToe::new();
        game.make_move(0, 'X');
        let available_moves = game.available_moves();
        assert_eq!(available_moves.len(), 8);
        assert!(!available_moves.contains(&0));
    }
}