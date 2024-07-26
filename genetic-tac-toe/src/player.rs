use crate::tic_tac_toe::TicTacToe;
use rand::seq::SliceRandom;

pub trait Player {
    fn get_move(&self, game: &TicTacToe) -> usize;
}

pub struct RandomComputerPlayer {
    letter: char,
}

impl RandomComputerPlayer {
    pub fn new(letter: char) -> Self {
        Self { letter }
    }
}

impl Player for RandomComputerPlayer {
    fn get_move(&self, game: &TicTacToe) -> usize {
        let moves = game.available_moves();
        *moves.choose(&mut rand::thread_rng()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_computer_player() {
        let player = RandomComputerPlayer::new('X');
        let mut game = TicTacToe::new();
        assert_eq!(game.get_board(), [' '; 9]);

        let move1_ = player.get_move(&game);
        assert!(game.make_move(move1_, 'X'));
        assert!(game.get_board().contains(&'X'));

        let move2_ = player.get_move(&game);
        assert!(game.make_move(move2_, 'X'));
        assert!(game.get_board().iter().filter(|&c| *c == 'X').count() >= 2);
    }
}