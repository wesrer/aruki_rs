use crate::{
    board::{Board, GameState},
    moves::{Position, PossibleMoves},
};

use super::{pieces::Pieces, Moves};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pawn {}

// TODO: promotions
impl Moves for Pawn {
    fn moves(pos: Position, board: &Board) -> Vec<PossibleMoves> {
        match board.player_piece(pos) {
            Some(Pieces::Pawn(_)) => {}
            _ => return vec![],
        }

        let row = pos.0;
        let col = pos.1;
        let move_config = (pos, Position(row + 1, col), board);

        if let Ok(x) = PossibleMoves::try_from(move_config) {
            return vec![x];
        } else {
            return vec![];
        }
    }
}
