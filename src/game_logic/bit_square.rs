use super::color::Color;
use super::square_state::SquareState;
use super::piece_type::PieceType;


/* Bit Layout (01234567):
 * 0-2 (012): SquareState. 000 = Empty, 001 = Pawn, 010 = Bishop, 011 = Knight, 100 = Rook, 101 = Queen, 110 = King, 111 is unused... panic!
 * 3: Color. 0 = White, 1 = Black. Ironic.
 * Bits 4-7 (4567) are Flags. Bits 4-6 could be merged, but there is no need. We have ample space.
 * 4: KING_HAS_MOVED. 0 = false, 1 = true. This bit should only be assigned for Kings.
 * 5: ROOK_HAS_MOVED. 0 = false, 1 = true. This bit should only be assigned for Rooks.
 * 6: EN_PASSANT_ABLE. 0 = false, 1 = true. This bit should only be assigned for Pawns. If the pawn lives to see the end of the next turn, it should be set back to false.
 * 7: CURRENT_TURN. 0 = White, 1 = Black. This bit should only be accessed for the first BitSquare in the BitBoard array. This bit should flip every move.
 * 
 * Needs to implement the From<SquareState> trait.
 */
#[derive(Clone, Copy)]
pub struct BitSquare {
   pub data: u8,
}

impl BitSquare {
    pub fn new(data: u8) -> Self {
        BitSquare { data }
    }


}

impl From<SquareState> for BitSquare {
    //Going from a Square<State> to a BitSquare is infallible.
    fn from(value: SquareState) -> Self {
        match value {
            SquareState::Empty => Self::new(0),
            SquareState::Occupied(piece) => {
                use PieceType::*;
                use Color::*;

                let mut flag_bits: u8 = 0;
                let role_bits: u8 = match piece.role {
                    Pawn(en_passant_able) => {
                        flag_bits = match en_passant_able {
                            true => 2, //0b0010
                            false => 0, //0b000
                        };
                        1 //0b001
                    }, 
                    Bishop => 2, //0b010
                    Knight => 3, //0b011
                    Rook(has_moved) => {
                        flag_bits = match has_moved {
                            true => 4, //0b0100
                            false => 0, //0b0000
                        };
                        4 //0b100
                    }, 
                    Queen => 5, //0b101
                    King(has_moved) => {
                        flag_bits = match has_moved {
                            true => 8, //0b1000
                            false => 0, //0b0000
                        };
                        6 //0b110
                    },
                    //7, aka 0b111 is not possible to represent with SquareState so we don't need to worry about it.
                };

                let color_bit: u8 = match piece.color {
                    White => 0, //0b0
                    Black => 1, //0b1
                };

                /* We never set the CURRENT_TURN flag since that is the responsibility of the Board, and not the SquareState.
                 * role_bits << 5 shifts role_bits up 5 bits into the correct place. We could also multiply by 32.
                 * color_bit << 4 shifts color_bit up 4 bits into the correct place. We could also multiply by 16.
                 * flag_bits was written to be placed at the end, so it doesn't need to be shifted.
                 */
                BitSquare::new(role_bits << 5 + color_bit << 4 + flag_bits)
            },
        }
    }
}



