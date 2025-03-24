//Will just impl move for the entire enum
#[derive(Clone, Copy)]
pub enum PieceType {
    //En passantable. See BitSquare for more logic
    Pawn(bool),
    Bishop,
    Knight,
    //Has moved. Ever.
    Rook(bool),
    Queen,
    //Same as Rook. For castling logic.
    King(bool),
}
