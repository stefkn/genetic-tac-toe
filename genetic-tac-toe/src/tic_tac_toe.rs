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

}