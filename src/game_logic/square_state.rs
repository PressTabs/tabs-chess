use super::{bit_square::BitSquare, piece::Piece, position::Position, color::Color, piece_type::PieceType};

//Basically an option
#[derive(Clone, Copy)]
pub enum SquareState {
    Empty,
    Occupied(Piece),
}

impl SquareState {
    /* Technically this is fallible if the underlying data is bad, but I'd rather it panic'd.
     * Also this is obviously not an actual From<BitSquare> method. That would be impossible since Piece requires a position, which BitSquare does not have sadly.
     * 
     * Bit Layout (01234567):
     * 0-2 (012): SquareState. 000 = Empty, 001 = Pawn, 010 = Bishop, 011 = Knight, 100 = Rook, 101 = Queen, 110 = King, 111 is unused... panic!
     * 3: Color. 0 = White, 1 = Black. Ironic.
     * Bits 4-7 (4567) are Flags. Bits 4-6 could be merged, but there is no need. We have ample space.
     * 4: KING_HAS_MOVED. 0 = false, 1 = true. This bit should only be assigned for Kings.
     * 5: ROOK_HAS_MOVED. 0 = false, 1 = true. This bit should only be assigned for Rooks.
     * 6: EN_PASSANT_ABLE. 0 = false, 1 = true. This bit should only be assigned for Pawns. If the pawn lives to see the end of the next turn, it should be set back to false.
     * 7: CURRENT_TURN. 0 = White, 1 = Black. This bit should only be accessed for the first BitSquare in the BitBoard array. This bit should flip every move.
     * 
     * Bit layout summary taken from BitSquare.
     */
    pub fn from_bit_square(value: BitSquare, pos: Position) -> Self {
        let role_bits = value.data >> 5;
        match role_bits {
            0 => SquareState::Empty, //0b000
            _ => SquareState::Occupied(Piece {
                role: match role_bits {
                    1 => PieceType::Pawn((value.data >> 1) % 2 == 0), //0b001
                    2 => PieceType::Bishop,
                    3 => PieceType::Knight,
                    4 => PieceType::Rook((value.data >> 2) % 2 == 0),
                    5 => PieceType::Queen,
                    6 => PieceType::King((value.data >> 3) % 2 == 0),
                    _ => panic!()
                },
                color: if (value.data >> 4) % 2 == 0 { Color::White } else { Color::Black },
                position: pos,
            }),
        }
    }
}

