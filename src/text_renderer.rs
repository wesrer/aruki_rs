use std::fmt::{write, Display, Formatter, Result as FmtResult};

use crate::{
    moves::{Moves, Position},
    pieces::{javelin::Javelin, king::King, pawn::Pawn},
    player::Player,
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

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut letters = COL_CHARS.chars();

        let row = self.0 + 1; // shift index so that players don't have to deal with 0 indexed moves
        let col = self.1;

        write!(f, "{}{}", letters.nth(col as usize).unwrap(), row)
    }
}

impl Display for Moves {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Move {
                piece,
                starting,
                ending,
                captured,
            } => match captured {
                Some(x) => write!(f, "{}({})x{}({})", piece.piece_type, starting, x, ending),
                None => write!(f, "{}({}){}", piece.piece_type, starting, ending),
            },
            Self::Evolution { piece, starting, ally_piece, ending_piece } => {
                write!(f, "{}({})+{}={}", piece.piece_type, starting, ally_piece, ending_piece)
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

impl Display for Pawn {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "P")
    }
}
