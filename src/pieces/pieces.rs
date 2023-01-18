use std::fmt::Display;

use enum_display_derive::Display;

use super::{
    arrow::Arrow, javelin::Javelin, king::King, lance::Lance, minister::Minister, pawn::Pawn,
    pike::Pike, rook::Rook, sword::Sword,
};

#[derive(Display, Clone, Copy, PartialEq)]
pub enum Pieces {
    King(King),
    Minister(Minister),
    Jester,
    Arrow(Arrow),
    Lance(Lance),
    GreaterLance,
    GoldenDragon,
    SilverDragon,
    Pike(Pike),
    GreaterPike,
    Sword(Sword),
    LongSword,
    Pawn(Pawn),
    Javelin(Javelin),
    Rook(Rook),
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

    pub fn arrow() -> Self {
        Self::Arrow(Arrow {})
    }

    pub fn rook() -> Self {
        Self::Rook(Rook {})
    }

    pub fn pike() -> Self {
        Self::Pike(Pike {})
    }

    pub fn greater_pike() -> Self {
        Self::GreaterPike
    }

    pub fn greater_lance() -> Self {
        Self::GreaterLance
    }

    pub fn lance() -> Self {
        Self::Lance(Lance {})
    }

    pub fn sword() -> Self {
        Self::Sword(Sword {})
    }

    pub fn longsword() -> Self {
        Self::LongSword
    }

    pub fn golden_dragon() -> Self {
        Self::GoldenDragon
    }

    pub fn silver_dragon() -> Self {
        Self::SilverDragon
    }

    pub fn jester() -> Self {
        Self::Jester
    }

    // NOTE: returns the first piece if no evolution is viable
    pub fn evolve(pieces: (Self, Self)) -> Self {
        match pieces {
            (Self::Pawn(_), Self::Pawn(_)) => Self::javelin(),
            (Self::Rook(_), Self::Arrow(_)) => Self::minister(),
            (Self::Arrow(_), Self::Rook(_)) => Self::minister(),
            (Self::Pike(_), Self::Pike(_)) => Self::greater_pike(),
            (Self::Lance(_), Self::Pike(_)) => Self::sword(),
            (Self::Pike(_), Self::Lance(_)) => Self::sword(),
            (Self::Sword(_), Self::Sword(_)) => Self::longsword(),
            (Self::GreaterLance, Self::GreaterPike) => Self::longsword(),
            (Self::GreaterPike, Self::GreaterLance) => Self::longsword(),
            _ => pieces.0,
        }
    }
}

impl TryFrom<&str> for Pieces {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "k" | "K" => Ok(Self::king()),
            "m" | "M" => Ok(Self::minister()),
            "a" | "A" => Ok(Self::arrow()),
            "r" | "R" => Ok(Self::rook()),
            "j" | "J" => Ok(Self::javelin()),
            "o" | "O" => Ok(Self::pawn()),
            "p" | "P" => Ok(Self::pike()),
            "gp" | "GP" => Ok(Self::greater_pike()),
            "l" | "L" => Ok(Self::lance()),
            "gl" | "GL" => Ok(Self::greater_lance()),
            "s" | "S" => Ok(Self::sword()),
            "ls" | "LS" => Ok(Self::longsword()),
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
