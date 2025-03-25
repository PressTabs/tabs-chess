
use game_logic::{bit_board::BitBoard, board::Board, color::Color, piece::Piece, piece_type::PieceType, position::Position, square_state::SquareState};

pub mod game_logic;

fn main() {
    println!("Board: {}, SquareState: {}, Piece: {}, PieceType: {}, Color: {}, Position: {}", 
            size_of::<Board>(),
            size_of::<SquareState>(),
            size_of::<Piece>(),
            size_of::<PieceType>(),
            size_of::<Color>(),
            size_of::<Position>(),
            );
}
