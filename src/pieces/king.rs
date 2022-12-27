use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::board::{Board, GameState};

use super::{Moves, pieces::Pieces};

#[derive(Clone, Copy)]
pub struct King {}

impl Display for King {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "K")
    }
}

// Needs to care about which piece is beside which
impl Moves for King {
    fn moves(pos: (u8, u8), board: Board) -> Vec<(u8, u8)> {
        // This situation should never occur, but guard clause nonetheless
        match board.player_piece(pos) {
            Some(Pieces::King(_)) => {}, 
            _ => return vec![],
        }

        let mut valid_moves = vec![
            (pos.0 + 1, pos.1), // directly forward
            (pos.0 - 1, pos.1), // directly backward
            (pos.0, pos.1 + 1), // directly to the right
            (pos.0, pos.1 - 1), // directly to the left
            (pos.0 + 1, pos.1 + 1), // diagonally right, up
            (pos.0 + 1, pos.1 - 1), // diagnoally left, up
            (pos.0 - 1, pos.1 + 1), // diagonally right, down
            (pos.0 - 1, pos.1 - 1), // diagonally left, down
        ];

        valid_moves.retain(Board::within_board);
        valid_moves.retain(|x| board.player_color(*x) != board.player_color(pos));
       
        valid_moves
    }
}