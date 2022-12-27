use crate::{
    board::{Board, GameState},
    moves::Position,
};

use super::{pieces::Pieces, Moves};

#[derive(Clone, Copy, PartialEq)]
pub struct Javelin {}

impl Moves for Javelin {
    fn moves(pos: Position, board: Board) -> Vec<Position> {
        match board.player_piece(pos) {
            Some(Pieces::Javelin(_)) => {}
            _ => return vec![],
        }

        let row = pos.0;
        let col = pos.1;

        let player_color = board.player_color(pos);

        let mut valid_moves = vec![];

        let temp_pos = Position(row + 1, col);

        if board.player_color(temp_pos) != player_color {
            valid_moves.push(temp_pos);

            let temp_pos = Position(row + 2, col);
            if board.player_color(temp_pos) != player_color {
                valid_moves.push(temp_pos);
            }
        }

        let temp_pos = Position(row - 1, col);

        if board.player_color(temp_pos) != player_color {
            valid_moves.push(temp_pos);

            let temp_pos = Position(row - 2, col);
            if board.player_color(temp_pos) != player_color {
                valid_moves.push(temp_pos);
            }
        }

        valid_moves.retain(Board::within_board);
        valid_moves.retain(|x| board.player_color(*x) != board.player_color(pos));

        valid_moves
    }
}
