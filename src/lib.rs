mod board;
mod errors;
mod game;
mod moves;
mod pieces;
mod player;
mod player_piece;

// Renderers
mod text_renderer;

pub const COL_CHARS: &'static str = "abcdefghijkl";

#[cfg(test)]
mod tests;
