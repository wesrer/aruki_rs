use crate::{
    board::{Board, BoardState},
    moves::{Move, Position},
};

use super::{pieces::Pieces, PieceProperties};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pawn {}

// TODO: promotions
impl PieceProperties for Pawn {
    fn moves(pos: Position, board: &Board) -> Vec<Move> {
        match board.player_piece(pos) {
            Some(Pieces::Pawn(_)) => {}
            _ => return vec![],
        }

        let row = pos.0;
        let col = pos.1;
        let move_config = (pos, Position(row + 1, col), board);

        if let Ok(x) = Move::try_from(move_config) {
            return vec![x];
        } else {
            return vec![];
        }
    }
}
