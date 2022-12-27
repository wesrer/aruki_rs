use crate::{
    board::{Board, GameState},
    moves::Position,
};

use super::{pieces::Pieces, Moves};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pawn {}

// TODO: promotions
impl Moves for Pawn {
    fn moves(pos: Position, board: Board) -> Vec<Position> {
        match board.player_piece(pos) {
            Some(Pieces::Pawn(_)) => {}
            _ => return vec![],
        }

        let mut valid_moves = vec![
            Position(pos.0 + 1, pos.1), // one forward
        ];

        valid_moves.retain(Board::within_board);
        valid_moves.retain(|x| board.player_color(*x) != board.player_color(pos));

        valid_moves
    }
}
