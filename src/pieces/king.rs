use crate::{
    board::{Board, GameState},
    moves::{Position, PossibleMoves},
};

use super::{lance::Lance, pieces::Pieces, pike::Pike, Moves};

#[derive(Clone, Copy, PartialEq)]
pub struct King {}

// Needs to care about which piece is beside which
impl Moves for King {
    fn moves(pos: Position, board: &Board) -> Vec<PossibleMoves> {
        // This situation should never occur, but guard clause nonetheless
        match board.player_piece(pos) {
            Some(Pieces::King(_)) => {}
            _ => return vec![], // TODO: refactor this into an Error
        }

        Lance::moves(pos, board)
            .into_iter()
            .chain(&mut Pike::moves(pos, board).into_iter())
            .collect()
    }
}
