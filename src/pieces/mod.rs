use enum_display_derive::Display;

use crate::player::Player;
use std::fmt::{Display, Formatter, Result as FmtResult};

use self::pieces::Pieces;

pub mod king;
pub mod pieces;

pub struct Piece {
    player: Player,
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
