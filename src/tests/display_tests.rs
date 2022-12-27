use crate::pieces::Piece;

#[test]
fn test_king() {
    let king_piece = Piece::new('w'.try_into().unwrap(), "k".try_into().unwrap());
    assert_eq!(format!("{}", king_piece), "W_K");

    let king_piece = Piece::new('b'.try_into().unwrap(), "k".try_into().unwrap());
    assert_eq!(format!("{}", king_piece), "B_K");
}

#[test]
fn test_minister() {
    let king_piece = Piece::new('w'.try_into().unwrap(), "m".try_into().unwrap());
    assert_eq!(format!("{}", king_piece), "W_M");

    let king_piece = Piece::new('b'.try_into().unwrap(), "m".try_into().unwrap());
    assert_eq!(format!("{}", king_piece), "B_M");
}