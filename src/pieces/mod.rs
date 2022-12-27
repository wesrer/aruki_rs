use crate::{player::Player, board::{GameState, Board}};
use std::fmt::{Display, Formatter, Result as FmtResult};

use self::pieces::Pieces;

pub mod pieces;
pub mod king;
pub mod minister;

pub struct Piece {
    pub player: Player,
    piece_type: Pieces,
}

impl Piece {
    pub fn new(player: Player, piece_type: Pieces) -> Self {
        Self { player, piece_type }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}_{}", self.player, self.piece_type)
    }
}

pub trait Moves { 
    fn moves(pos: (u8, u8), board: Board) -> Vec<(u8, u8)>;
    fn is_valid_move(pos: (u8, u8), mov: (u8, u8), board: Board) -> bool {
        Self::moves(pos, board).contains(&mov)
    }
}