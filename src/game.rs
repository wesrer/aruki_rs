use crate::player::Player;

pub struct Game {}

pub enum GameStates {
    Check(Player), // The Player who is currently under check
    Stalemate,
    Victory(Player), // TODO: differentiate resign?
    Playing,
    Draw, // through offer?
}
