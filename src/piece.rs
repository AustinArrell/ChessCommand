use crate::util::Pos;

pub trait Piece{
    fn verify_move(&self,pos1:&Pos, pos2:&Pos) -> bool;
    fn get_identity(&self) -> char;
}