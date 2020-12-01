use crate::piece::Piece;
use crate::util::Pos;

pub struct Knight{
    pub is_white: bool,
    pub identity: char,
}

impl Knight{
    pub fn new(is_white:bool, identity: char) -> Self
    {
        return Knight{is_white,identity};
    }
}

impl Piece for Knight{
    fn verify_move(&self,pos1:&Pos, pos2:&Pos) -> bool
    //Takes in two positions on the board and returns if that is a valid move for this identity
    {
        return true;
    }

    fn get_identity(&self) -> char
    {
        return self.identity;
    }
}