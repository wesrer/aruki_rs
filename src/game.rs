use std::fs::read_to_string;
use std::path::Path;

use crate::board::setup::BoardSetup;
use crate::board::Board;
use crate::errors::GameError;
use crate::moves::Move;
use crate::player::Player;

pub struct Game {
    pub state: GameStates,
    pub history: Vec<Move>,
}

impl Game {
    /// Reads a file that has a game saved on it and generates a game state that can be interacted with
    pub fn from_file(file: &Path) -> Result<Self, GameError> {
        let file_string = read_to_string(file)
            .map_err(|_| GameError::UnableToProcessFile("File not found".to_owned()))?;

        Self::from_string(file_string)
    }

    /// Converts from a recorded game into a game state that can be interacted with
    pub fn from_string(string: String) -> Result<Self, GameError> {
        todo!()
    }

    pub fn get_final_state(&self) -> Board {
        let mut board = Board::starting_pos();

        for m in &self.history {}
        board
    }
}

pub enum GameStates {
    Check(Player), // The Player who is currently under check
    Stalemate,
    Victory(Player), // TODO: differentiate resign?
    Playing,
    Draw, // through offer?
}
