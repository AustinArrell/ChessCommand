use crate::piece::Piece;
use crate::util::Pos;

pub struct King{
    pub white: bool,
    pub identity: char,
}

impl King{
    pub fn new(white:bool) -> Self
    {
        return King{white,identity: 'K'};
    }
}

impl Piece for King{
    fn verify_move(&self,pos1:&Pos, pos2:&Pos) -> bool
    //Takes in two positions on the board and returns if that is a valid move for this identity
    {
        let rank_dif = (pos2.rank - pos1.rank).abs();
        let file_dif = (pos2.file - pos1.file).abs();

        return rank_dif <= 1 && file_dif <= 1;
    }

    fn get_identity(&self) -> char
    {
        return self.identity;
    }

    fn is_white(&self) -> bool
    { self.white }
}



