use crate::{
    board::{Board, GameState},
    moves::Position,
};

use super::{pieces::Pieces, Moves};

#[derive(Clone, Copy, PartialEq)]
pub struct Javelin {}

impl Moves for Javelin {
    fn moves(pos: Position, board: Board) -> Vec<Position> {
        match board.player_piece(pos) {
            Some(Pieces::Javelin(_)) => {}
            _ => return vec![],
        }

        let row = pos.0;
        let col = pos.1;

        let mut valid_moves = vec![
            Position(row + 1, col), // one forward
            Position(row + 2, col), // two forward
            Position(row - 1, col), // one backward
            Position(row - 2, col), // two backward
        ];

        valid_moves.retain(Board::within_board);
        valid_moves.retain(|x| board.player_color(*x) != board.player_color(pos));

        valid_moves
    }
}
