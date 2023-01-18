use crate::{
    board::{Board, BoardState},
    moves::{Move, Position},
};

use super::{pieces::Pieces, PieceProperties};

#[derive(Clone, Copy, PartialEq)]
pub struct Pike {}

impl PieceProperties for Pike {
    fn moves(pos: Position, board: &Board) -> Vec<Move> {
        match board.player_piece(pos) {
            Some(Pieces::Pike(_)) => {}
            _ => return vec![],
        }

        let row = pos.0;
        let col = pos.1;

        let valid_moves = vec![
            Position(row + 1, col + 1), // topright diagonal
            Position(row + 1, col - 1), // topleft diagonal
            Position(row - 1, col + 1), // bottomright diagonal
            Position(row - 1, col - 1), // bottomleft diagonal
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
