use crate::{
    board::Board,
    moves::{Move, Position},
};

pub mod arrow;
pub mod javelin;
pub mod king;
pub mod lance;
pub mod minister;
pub mod pawn;
pub mod pieces;
pub mod pike;
pub mod rook;
pub mod sword;

pub trait PieceProperties {
    fn moves(pos: Position, board: &Board) -> Vec<Move>;
    fn is_valid_move(pos: Position, mov: Position, board: Board) -> bool {
        todo!()
        // Self::moves(pos, board).contains(&mov)
    }
}
