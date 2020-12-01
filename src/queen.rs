use crate::piece::Piece;
use crate::util::Pos;

pub struct Queen{
    pub is_white: bool,
    pub identity: char,
}

impl Queen{
    pub fn new(is_white:bool) -> Self
    {
        return Queen{is_white,identity:'Q'};
    }
}

impl Piece for Queen{
    fn verify_move(&self,pos1:&Pos, pos2:&Pos) -> bool
    //Takes in two positions on the board and returns if that is a valid move for this identity
    {
        //The queens movement is simply the rook and the bishop combined
        let file_dif = (pos2.file - pos1.file).abs();
        let rank_dif = (pos2.rank - pos1.rank).abs();
        
        return file_dif == rank_dif || (pos1.file == pos2.file || pos1.rank == pos2.rank);
        
    }

    fn get_identity(&self) -> char
    {
        return self.identity;
    }
}