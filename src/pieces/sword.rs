use crate::{
    board::{Board},
    moves::{Position, PossibleMoves},
};

use super::{Moves, lance::Lance, pike::Pike};

#[derive(Clone, Copy, PartialEq)]
pub struct Sword {}

// Needs to care about which piece is beside which
impl Moves for Sword {
    fn moves(pos: Position, board: &Board) -> Vec<PossibleMoves> {
        Lance::moves(pos, board)
            .into_iter()
            .chain(&mut Pike::moves(pos, board).into_iter())
            .collect()
    }
}
