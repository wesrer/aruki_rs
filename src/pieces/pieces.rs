use std::fmt::Display;

use enum_display_derive::Display;

use super::{javelin::Javelin, king::King, minister::Minister, pawn::Pawn};

#[derive(Display, Clone, Copy, PartialEq)]
pub enum Pieces {
    King(King),
    Minister(Minister),
    // Jester,
    Arrow,
    // Lance,
    // GreaterLance,
    // GreaterRiver,
    // LesserRiver,
    // Pike,
    // GreaterPike,
    // Sword,
    // LongSword,
    Pawn(Pawn),
    Javelin(Javelin),
    Rook,
}

impl Pieces {
    // pub fn is_valid_move(&self, pos: Position)

    pub fn king() -> Self {
        Self::King(King {})
    }

    pub fn minister() -> Self {
        Self::Minister(Minister {})
    }

    pub fn javelin() -> Self {
        Self::Javelin(Javelin {})
    }

    pub fn pawn() -> Self {
        Self::Pawn(Pawn {})
    }

    pub fn evolve(pieces: (Self, Self)) -> Self {
        match pieces {
            (Self::Pawn(_), Self::Pawn(_)) => Self::javelin(),
            (Self::Rook, Self::Arrow) => Self::minister(),
            _ => unimplemented!(),
        }
    }
}

impl TryFrom<&str> for Pieces {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "k" | "K" => Ok(Self::king()),
            "m" | "M" => Ok(Self::minister()),
            "p" | "P" => Ok(Self::pawn()),
            _ => Err("invalid character"),
        }
    }
}

impl TryFrom<String> for Pieces {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}
