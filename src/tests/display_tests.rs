use crate::{
    moves::{Moves, Position},
    pieces::{pawn, pieces::Pieces},
    player_piece::PlayerPiece,
};

#[test]
fn test_king() {
    let king_piece = PlayerPiece::new('w'.try_into().unwrap(), "k".try_into().unwrap());
    assert_eq!(format!("{}", king_piece), "W_K");

    let king_piece = PlayerPiece::new('b'.try_into().unwrap(), "k".try_into().unwrap());
    assert_eq!(format!("{}", king_piece), "B_K");
}

#[test]
fn test_minister() {
    let king_piece = PlayerPiece::new('w'.try_into().unwrap(), "m".try_into().unwrap());
    assert_eq!(format!("{}", king_piece), "W_M");

    let king_piece = PlayerPiece::new('b'.try_into().unwrap(), "m".try_into().unwrap());
    assert_eq!(format!("{}", king_piece), "B_M");
}

#[test]
fn test_render_normal_move() {
    let king_piece = PlayerPiece::new('w'.try_into().unwrap(), "k".try_into().unwrap());
    let king_move = Moves::Move {
        piece: king_piece,
        starting: Position(0, 0),
        ending: Position(1, 0),
        captured: None,
    };

    assert_eq!(format!("{}", king_move), "K(a1)a2");
}

#[test]
fn test_render_capture_move() {
    let king_piece = PlayerPiece::new('w'.try_into().unwrap(), "k".try_into().unwrap());
    let king_move = Moves::Move {
        piece: king_piece,
        starting: Position(0, 0),
        ending: Position(1, 0),
        captured: Some(Pieces::pawn()),
    };

    assert_eq!(format!("{}", king_move), "K(a1)xP(a2)");
}

#[test]
fn test_render_evolution_move() {
    let pawn_piece = PlayerPiece::new('w'.try_into().unwrap(), "p".try_into().unwrap());
    let pawn_evolve = Moves::Evolution { piece: pawn_piece, starting: Position(1,1), ally_piece: Pieces::pawn(), ending_piece: Pieces::javelin() };

    assert_eq!(format!("{}", pawn_evolve), "P(a2)+P=J");

}