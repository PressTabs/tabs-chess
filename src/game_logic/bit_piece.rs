
//See PieceBoard for details.
//This struct only exists to implement Ord on to get .sort() functionality.

//According to our data convention, this actually doesn't need any extra logic to Ord properly. Nice!
//Because our first three bits are the rank, and the next three are the file. This should sort without regard to role, only to position, just fine.
//We just need to be careful to only sort in the correct chunk sizes.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct BitPiece {
    pub data: u8
}
