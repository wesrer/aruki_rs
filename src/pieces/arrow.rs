use crate::{
    board::{Board, BoardState},
    moves::{Move, MoveConfig, Position},
    player,
};

use super::{pieces::Pieces, PieceProperties};

#[derive(Clone, Copy, PartialEq)]
pub struct Arrow {}

// Needs to care about which piece is beside which
impl PieceProperties for Arrow {
    fn moves(pos: Position, board: &Board) -> Vec<Move> {
        // This situation should never occur, but guard clause nonetheless
        match board.player_piece(pos) {
            Some(Pieces::Arrow(_)) => {}
            _ => return vec![], // TODO: refactor this into an Error
        }

        let mut moves = vec![];

        let (mut row, mut col) = (pos.0, pos.1);

        // Upper right diagonal
        while row < 11 && col < 11 {
            row += 1;
            col += 1;
            let move_config = (pos, Position(row, col), board);
            match board.live_board[row as usize][col as usize] {
                None => moves.push(move_config.try_into().unwrap()),
                Some(x) => {
                    // You can capture the piece you encountered if it's the opponent's piece, but that's the furthest you can go in this direction
                    // If you find your own piece, you have already reached the furthest point in this direction
                    if x.player != board.player_color(pos).unwrap() {
                        moves.push(move_config.try_into().unwrap());
                    }
                    break;
                }
            }
        }

        // Lower right diagonal
        let (mut row, mut col) = (pos.0, pos.1);
        while row > 0 && col < 11 {
            row -= 1;
            col += 1;
            let move_config = (pos, Position(row, col), board);
            match board.live_board[row as usize][col as usize] {
                None => moves.push(move_config.try_into().unwrap()),
                Some(x) => {
                    // You can capture the piece you encountered if it's the opponent's piece, but that's the furthest you can go in this direction
                    // If you find your own piece, you have already reached the furthest point in this direction
                    if x.player != board.player_color(pos).unwrap() {
                        moves.push(move_config.try_into().unwrap());
                    }
                    break;
                }
            }
        }

        // Upper left diagonal
        let (mut row, mut col) = (pos.0, pos.1);
        while row < 11 && col > 0 {
            row += 1;
            col -= 1;
            let move_config = (pos, Position(row, col), board);
            match board.live_board[row as usize][col as usize] {
                None => moves.push(move_config.try_into().unwrap()),
                Some(x) => {
                    // You can capture the piece you encountered if it's the opponent's piece, but that's the furthest you can go in this direction
                    // If you find your own piece, you have already reached the furthest point in this direction
                    if x.player != board.player_color(pos).unwrap() {
                        moves.push(move_config.try_into().unwrap());
                    }
                    break;
                }
            }
        }

        // Lower left diagonal
        let (mut row, mut col) = (pos.0, pos.1);
        while row > 0 && col > 0 {
            row -= 1;
            col -= 1;
            let move_config = (pos, Position(row, col), board);
            match board.live_board[row as usize][col as usize] {
                None => moves.push(move_config.try_into().unwrap()),
                Some(x) => {
                    // You can capture the piece you encountered if it's the opponent's piece, but that's the furthest you can go in this direction
                    // If you find your own piece, you have already reached the furthest point in this direction
                    if x.player != board.player_color(pos).unwrap() {
                        moves.push(move_config.try_into().unwrap());
                    }
                    break;
                }
            }
        }

        moves
    }
}
