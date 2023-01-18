use crate::{
    board::{Board, BoardState},
    moves::{Move, Position},
};

use super::PieceProperties;

#[derive(Clone, Copy, PartialEq)]
pub struct Lance {}

// Needs to care about which piece is beside which
impl PieceProperties for Lance {
    fn moves(pos: Position, board: &Board) -> Vec<Move> {
        // cannot have guard clauses since many other pieces are expected to
        // reuse this moveset (King, Sword, GreaterLance, LongSword)

        let row = pos.0;
        let col = pos.1;

        let valid_moves = vec![
            Position(row + 1, col), // directly forward
            Position(row - 1, col), // directly backward (pos.0, pos.1 + 1),
            Position(row, col - 1), // directly to the left
            Position(row, col + 1), // directly to the right
        ];

        let valid_moves = valid_moves
            .iter()
            .filter(|pos| Board::within_board(*pos))
            .filter(|pos| board.player_color(**pos) != board.player_color(**pos))
            .map(|pos| {
                let move_config = (Position(row, col), *pos, board);
                Move::try_from(move_config).unwrap() // This is fine because this unwrap should never trigger
            })
            .collect();

        valid_moves
    }
}
