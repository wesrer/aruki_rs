use crate::{
    board::{Board, BoardState},
    moves::{Move, Position},
};

use super::{lance::Lance, pieces::Pieces, pike::Pike, PieceProperties};

#[derive(Clone, Copy, PartialEq)]
pub struct King {}

// Needs to care about which piece is beside which
impl PieceProperties for King {
    fn moves(pos: Position, board: &Board) -> Vec<Move> {
        // This situation should never occur, but guard clause nonetheless
        match board.player_piece(pos) {
            Some(Pieces::King(_)) => {}
            _ => return vec![], // TODO: refactor this into an Error
        }

        // A King's move is a combination of the moves of a Lance and a Pike
        Lance::moves(pos, board)
            .into_iter()
            .chain(&mut Pike::moves(pos, board).into_iter())
            .collect()
    }
}
