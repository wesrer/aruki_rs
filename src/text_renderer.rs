use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    moves::{Move, Position},
    pieces::{
        javelin::Javelin, king::King, lance::Lance, pawn::Pawn, pike::Pike, rook::Rook,
        sword::Sword,
    },
    player::Player,
    player_piece::PlayerPiece,
    COL_CHARS,
};

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let letter = match self {
            Self::White => 'W',
            Self::Black => 'B',
        };
        write!(f, "{}", letter)
    }
}

impl Display for PlayerPiece {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}_{}", self.player, self.piece_type)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut letters = COL_CHARS.chars();

        let row = self.0 + 1; // shift index so that players don't have to deal with 0 indexed moves
        let col = self.1;

        write!(f, "{}{}", letters.nth(col as usize).unwrap(), row)
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::MovePiece {
                piece,
                starting,
                ending,
                captured,
            } => match captured {
                Some(x) => write!(f, "{}({})x{}({})", piece.piece_type, starting, x, ending),
                None => write!(f, "{}({}){}", piece.piece_type, starting, ending),
            },
            Self::Evolution {
                piece,
                starting,
                ally_piece,
                ending_piece,
            } => {
                write!(
                    f,
                    "{}({})+{}={}",
                    piece.piece_type, starting, ally_piece, ending_piece
                )
            }
            _ => unimplemented!(),
        }
    }
}

// Text renderings for indvidual pieces

impl Display for King {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "K")
    }
}

impl Display for Javelin {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "J")
    }
}

impl Display for Rook {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "R")
    }
}

impl Display for Pawn {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "O")
    }
}

impl Display for Pike {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "P")
    }
}

impl Display for Lance {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "L")
    }
}

impl Display for Sword {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "S")
    }
}
