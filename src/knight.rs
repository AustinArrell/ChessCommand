use crate::piece::Piece;
use crate::util::Pos;

pub struct Knight{
    pub is_white: bool,
    pub identity: char,
}

impl Knight{
    pub fn new(is_white:bool) -> Self
    {
        return Knight{is_white,identity:'N'};
    }
}

impl Piece for Knight{
    fn verify_move(&self,pos1:&Pos, pos2:&Pos) -> bool
    //Takes in two positions on the board and returns if that is a valid move for this identity
    {
        let rank_dif = (pos2.rank - pos1.rank).abs();
        let file_dif = (pos2.file - pos1.file).abs();
        let distance_moved = (((pos2.file - pos1.file).pow(2) as f32) + ((pos2.rank - pos1.rank).pow(2)as f32)).sqrt();
        
        return (file_dif == rank_dif+1 || file_dif == rank_dif-1) && distance_moved as i32 == 2
    }

    fn get_identity(&self) -> char
    {
        return self.identity;
    }
}