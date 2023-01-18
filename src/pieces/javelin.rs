use crate::{
    board::{Board, BoardState},
    moves::{Move, Position},
};

use super::{pieces::Pieces, PieceProperties};

#[derive(Clone, Copy, PartialEq)]
pub struct Javelin {}

impl PieceProperties for Javelin {
    fn moves(pos: Position, board: &Board) -> Vec<Move> {
        match board.player_piece(pos) {
            Some(Pieces::Javelin(_)) => {}
            _ => return vec![],
        }

        let row = pos.0;
        let col = pos.1;

        let mut valid_moves = vec![];

        let one_forward = Position(row + 1, col);
        if let Ok(of) = Move::try_from((pos, one_forward, board)) {
            valid_moves.push(of);

            let two_forward = Position(row + 2, col);
            if let Ok(tf) = Move::try_from((pos, two_forward, board)) {
                valid_moves.push(tf);
            }
        }

        let one_backward = Position(row - 1, col);
        if let Ok(of) = Move::try_from((pos, one_backward, board)) {
            valid_moves.push(of);

            let two_backward = Position(row - 2, col);
            if let Ok(tf) = Move::try_from((pos, two_backward, board)) {
                valid_moves.push(tf);
            }
        }

        valid_moves
    }
}
