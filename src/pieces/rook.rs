use crate::{
    board::{Board, GameState},
    moves::{Position, PossibleMoves},
};

use super::{pieces::Pieces, Moves};

#[derive(Clone, Copy, PartialEq)]
pub struct Rook {}

// Needs to care about which piece is beside which
impl Moves for Rook {
    fn moves(pos: Position, board: Board) -> Vec<PossibleMoves> {
        // This situation should never occur, but guard clause nonetheless
        match board.player_piece(pos) {
            Some(Pieces::King(_)) => {}
            _ => return vec![], // TODO: refactor this into an Error
        }

        let mut valid_moves = vec![];

        let row = pos.0;
        let col = pos.1;

        // upwards
        for c in (col + 1)..12 {
            let new_pos = Position(row, c);
            match PossibleMoves::try_from((pos, Position(row, c), board)) {
                Ok(x) => valid_moves.push(x),
                Err(_) => break,
            }
        }

        // downwards
        for c in 0..col {
            let new_pos = Position(row, c);
            match PossibleMoves::try_from((pos, Position(row, c), board)) {
                Ok(x) => valid_moves.push(x),
                Err(_) => break,
            }
        }
        // left
        for r in 0..row {
            let new_pos = Position(r, col);
            match PossibleMoves::try_from((pos, Position(r, col), board)) {
                Ok(x) => valid_moves.push(x),
                Err(_) => break,
            }
        }

        // right
        for r in (row + 1)..12 {
            let new_pos = Position(r, col);
            match PossibleMoves::try_from((pos, Position(r, col), board)) {
                Ok(x) => valid_moves.push(x),
                Err(_) => break,
            }
        }

        valid_moves
    }
}
