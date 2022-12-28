use crate::{
    board::Board,
    moves::{Position, PossibleMoves},
};

pub mod javelin;
pub mod king;
pub mod lance;
pub mod minister;
pub mod pawn;
pub mod pieces;
pub mod pike;
pub mod rook;
pub mod sword;

pub trait Moves {
    fn moves(pos: Position, board: &Board) -> Vec<PossibleMoves>;
    fn is_valid_move(pos: Position, mov: Position, board: Board) -> bool {
        todo!()
        // Self::moves(pos, board).contains(&mov)
    }
}
