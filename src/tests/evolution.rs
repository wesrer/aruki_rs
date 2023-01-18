use crate::{
    board::{setup::BoardSetup, Board},
    moves::Position,
    player_piece::PlayerPiece,
};

#[test]
pub fn test_pawn_evolution() {
    let white_pieces = [("o".try_into().unwrap(), Position(4, 4))];
    let white_captured = ["o".try_into().unwrap()];
    let mut board = Board::with_pieces(white_pieces, white_captured, [], []);

    let expected_square = Some(PlayerPiece::new(
        'w'.try_into().unwrap(),
        "j".try_into().unwrap(),
    ));
    assert_eq!(board.live_board[4][4], expected_square);
}
