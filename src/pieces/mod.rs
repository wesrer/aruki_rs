use crate::{board::Board, moves::Position};

pub mod javelin;
pub mod king;
pub mod minister;
pub mod pawn;
pub mod pieces;

pub trait Moves {
    fn moves(pos: Position, board: Board) -> Vec<Position>;
    fn is_valid_move(pos: Position, mov: Position, board: Board) -> bool {
        Self::moves(pos, board).contains(&mov)
    }
}
