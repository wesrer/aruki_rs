use crate::{pieces::pieces::Pieces, player::Player};

use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy)]
pub struct PlayerPiece {
    pub player: Player,
    pub piece_type: Pieces,
}

impl PlayerPiece {
    pub fn new(player: Player, piece_type: Pieces) -> Self {
        Self { player, piece_type }
    }
}

impl Display for PlayerPiece {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}_{}", self.player, self.piece_type)
    }
}
