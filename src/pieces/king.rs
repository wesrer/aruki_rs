use crate::{
    board::{Board, GameState},
    moves::{Position, PossibleMoves},
};

use super::{pieces::Pieces, Moves};

#[derive(Clone, Copy, PartialEq)]
pub struct King {}

// Needs to care about which piece is beside which
impl Moves for King {
    fn moves(pos: Position, board: Board) -> Vec<PossibleMoves> {
        // This situation should never occur, but guard clause nonetheless
        match board.player_piece(pos) {
            Some(Pieces::King(_)) => {}
            _ => return vec![], // TODO: refactor this into an Error
        }

        let row = pos.0;
        let col = pos.1;

        let mut valid_moves = vec![
            Position(row + 1, col),     // directly forward
            Position(row - 1, col),     // directly backward (pos.0, pos.1 + 1),
            Position(row, col - 1),     // directly to the left
            Position(row, col + 1),     // directly to the right
            Position(row + 1, col + 1), // diagonally right, up
            Position(row + 1, col - 1), // diagnoally left, up
            Position(row - 1, col + 1), // diagonally right, down
            Position(row - 1, col - 1), // diagonally left, down
        ];

        let valid_moves = valid_moves
            .iter()
            .filter(|pos| Board::within_board(*pos))
            .filter(|pos| board.player_color(**pos) != board.player_color(**pos))
            .map(|pos| {
                let move_config = (Position(row, col), *pos, board);
                PossibleMoves::try_from(move_config).unwrap() // This is fine because this unwrap should never trigger
            })
            .collect();

        valid_moves
    }
}
