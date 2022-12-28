use crate::{
    board::Board,
    moves::{Position, PossibleMoves},
};

pub mod javelin;
pub mod king;
pub mod minister;
pub mod pawn;
pub mod pieces;
pub mod rook;

pub trait Moves {
    fn moves(pos: Position, board: Board) -> Vec<PossibleMoves>;
    fn is_valid_move(pos: Position, mov: Position, board: Board) -> bool {
        todo!()
        // Self::moves(pos, board).contains(&mov)
    }
}
