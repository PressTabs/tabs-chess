/*
 * Our BitBoard representation is really neat. It only uses 64 bytes. With some sacrifices though, we can go smaller.
 * There are a max of 32 pieces in play on a chess board at any given time. 
 * 
 * We can identify them as follows:
 *  - The first half is white. The second half is black.
 *  - The first eight of each half are pawns. The next two are bishops. The next two are knights. The next two are rooks. Finally, the last are the queen and king in that order.
 * 
 * We've made a scheme now to identify pieces from an ordered array, string, whatever, of individualized data.
 * 
 * Each piece will need to store it's position however, unlike in the BitBoard representation. 
 * A chessboard is 8x8. We thus need 3 bits (2^3 = 8) for both the rank and the file. That gives us 6 bits so far.
 * 
 * We'll use a seventh bit as a union for the KING_HAS_MOVED, ROOK_HAS_MOVED, and EN_PASSANT_ABLE (bits 4-6 in the BitSquare documentation) as these are mutually exclusive.
 * 
 * Finally, we'll use the eighth bit to mark if a piece has been removed from play (died).
 * 
 * You may have noticed a problem. How will we mark whose turn it is? The BitSquare representation had a single bit in the first square as an access bit for this case, but
 * we seemingly have no recourse unless we want to go from 8 bits (yay!) to 9 bits (ew!). Well, there is a solution.
 * 
 * The seventh bit is very important for encoding the more esoteric rules of chess. Castling and en passant haven't always existed. Pawns, Rooks, and Kings all rely on this
 * bit for proper function. Bishops, Knights, and Queens couldn't care less. This bit doesn't effect their function, and that's the answer to our problem. We can simply add
 * our CURRENT_TURN bit into the seventh bit union and designate some *magic* bishop, knight, or queen to carry it. I think it would be fitting if the White Queen carried it,
 * so that is how I will implement it.
 * 
 * This implementation carries several drawbacks in exchange for being half the size. Putting aside how cumbersome working with this format would be, the BitBoard has one
 * really nice bonus: all identical states have the same bit representation. This means that checking if two game states (boards) are identical, for the sake of alpha-beta
 * pruning, is as easy as using an == operator. This is unfortunately not the case for this representation.
 * 
 * According to this representation we identify by the identify of the piece. This is not the case for any other representation. You thus might imagine a situation where the
 * knights swapped positions and such. Though the game state is identical, this representation would not suggest they are using just the == operator without implementing the Eq
 * trait differently. We can in fact do that however. It is just more complex, both to implement and in terms of computational power required to verify equality.
 * 
 * I pondered for a while the various ways this could be done. There are 8! permutations of pawns, 2! permutations of bishops, knights, and rooks, all for just one half. That's
 * an unforgivable amount of computation, especially just for one half. There is an incredible naive, working (I think) computationally solid solution. 
 * 
 * NAIVE SOLUTION #1:
 * 
 * Our data is ordered. This is what lets us perform this operation to begin with. Our other solutions seperate the identity of a piece from it's ordering in the data.
 * Instead, we can ensure our data is ALWAYS ordered in an even more specific way to gurantee bit equality. The first set of conventions we applied are great, but
 * we can be even more specific.
 * 
 * DEFINE A NEW CONVENTION:
 * 
 *  - Let each type of piece, for each color, be ALWAYS sorted by position with RANK being the primary index and FILE being the secondary index. 
 * 
 * This convention **gurantees** that two identical game states will always have the same PieceBoard representation. Furthermore, as pieces move around and their positions change, 
 * the PieceBoard will need to be sorted again. This forbids us from storing the CURRENT_TURN bit in a bishop or knight since their order may swap and we won't know which one to
 * check for the magic bit (technically you can check both and if neither have it, it's 0, and if one has it, it's 1, but that's expensive). Since there is only one queen however,
 * our current convention of storing the CURRENT_TURN bit in the white queen (it would work equally well in the black queen) holds without need for modification.
 * 
 * Great, now we just need to implement it.
 * 
 * ADDENDUM CONVENTION:
 * 
 * To make sorting logic easier, faster, etc., all dead pieces MUST be notated with all bits set to 1, i.e. the byte value must be 255. This also makes it very easy to check for, and
 * subsequently ignore dead pieces when rendering, performing game logic, and so on.
 * 
 * PROBLEM: THIS CANNOT HANDLE PROMOTION. I WILL NEED TO RESOLVE THIS.
 * SOLUTION CANDIDATE: SET THE PAWN TO DEAD. Set the magic bit to off. This is against convention for dead pieces and should thus be recognizable.
 *                     This will trigger a backtrace to figure out what a pawn has been promoted to. Alternatively engage a new packing convention.
 * SOLUTION CANDIDATE: SET THE MAGIC BIT IN THE BLACK QUEEN. (FAIL)
 * 
 * PROBLEM: DEATH CONVENTION CONFLICTS WITH QUEEN BIT HACK
 * SOLUTION: QUEENS ARE BY THEMSELVES. THE QUEEN BIT HACK WILL WORK.
 * 
 * PROBLEM: PROMOTION EXPANDS THE SIZE OF EACH PieceType + Color Section. We cannot support this
 * SOLUTION: Either a. forgo this representation, or b. As soon as there is a promotion flip the Black Queen's magic bit (PROMOTION_REACHED) and dynamically switch our representation,
 *           starting at that point in the tree, to the BitBoard representation.
 */

use super::bit_piece::BitPiece;

#[derive(Clone, Copy)]
 pub struct PieceBoard {
    pub data: [BitPiece; 32]
 }

 impl PieceBoard {
    pub const fn new() -> Self {
        let mut piece_board = [0u8; 32];
        todo!()
    }
 }