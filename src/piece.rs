enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

enum Color {
    White,
    Black,
}

struct Piece {
    Type: PieceType,
    Color: Color,
}