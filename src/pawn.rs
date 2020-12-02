use crate::piece::Piece;
use crate::util::Pos;

pub struct Pawn
{
    pub white: bool,
    pub identity: char,
}

impl Pawn
{
    pub fn new(white:bool) -> Self
    { Pawn {white,identity:'P'} }
}

impl Piece for Pawn{
    fn verify_move(&self,pos1:&Pos, pos2:&Pos) -> bool
    //Takes in two positions on the board and returns if that is a valid move for this identity
    {
        if self.white
        { pos2.file == pos1.file+1 && pos2.rank == pos1.rank }
        else
        { pos2.file == pos1.file-1 && pos2.rank == pos1.rank }
    }

    fn get_identity(&self) -> char
    { self.identity }

    fn is_white(&self) -> bool
    { self.white }
}