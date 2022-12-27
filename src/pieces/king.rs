use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    board::{Board, GameState},
    moves::Position,
};

use super::{pieces::Pieces, Moves};

#[derive(Clone, Copy, PartialEq)]
pub struct King {}

// Needs to care about which piece is beside which
impl Moves for King {
    fn moves(pos: Position, board: Board) -> Vec<Position> {
        // This situation should never occur, but guard clause nonetheless
        match board.player_piece(pos) {
            Some(Pieces::King(_)) => {}
            _ => return vec![], // TODO: refactor this into an Error
        }

        let row = pos.0;
        let col = pos.1;

        let mut valid_moves = vec![
            Position(row + 1, col),     // directly forward
            Position(row - 1, col), // directly backward (pos.0, pos.1 + 1),     // directly to the right
            Position(row, col - 1), // directly to the left
            Position(row, col + 1), // directly to the right
            Position(row + 1, col + 1), // diagonally right, up
            Position(row + 1, col - 1), // diagnoally left, up
            Position(row - 1, col + 1), // diagonally right, down
            Position(row - 1, col - 1), // diagonally left, down
        ];

        valid_moves.retain(Board::within_board);
        valid_moves.retain(|x| board.player_color(*x) != board.player_color(pos));

        valid_moves
    }
}
