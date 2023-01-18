use crate::{
    board::Board,
    moves::{Move, Position},
};

use super::{lance::Lance, pike::Pike, PieceProperties};

#[derive(Clone, Copy, PartialEq)]
pub struct Sword {}

// Needs to care about which piece is beside which
impl PieceProperties for Sword {
    fn moves(pos: Position, board: &Board) -> Vec<Move> {
        Lance::moves(pos, board)
            .into_iter()
            .chain(&mut Pike::moves(pos, board).into_iter())
            .collect()
    }
}
