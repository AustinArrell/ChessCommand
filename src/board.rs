use crate::piece::Piece;

use crate::pawn::Pawn;
use crate::rook::Rook;
use crate::knight::Knight;
use crate::bishop::Bishop;
use crate::king::King;
use crate::queen::Queen;
use crate::air::Air;

use crate::util::{Pos,cons,cons::Color};

/*
Rank: A-H Queen - King
File: 1-8 White - Black
*/

pub struct Board
{
    grid : [[Box<dyn Piece>;8];8],

    white_king : Pos,
    black_king : Pos,

    black_col : Color,
    white_col : Color,
    black_tile_col : Color,
    white_tile_col : Color,
}

impl Board
{
    pub fn new() -> Self
    {
        Board
        {
            grid:
            [
                [Box::new(Rook::new(true)),   Box::new(Pawn::new(true)), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Pawn::new(false)), Box::new(Rook::new(false))],
                [Box::new(Knight::new(true)), Box::new(Pawn::new(true)), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Pawn::new(false)), Box::new(Knight::new(false))],
                [Box::new(Bishop::new(true)), Box::new(Pawn::new(true)), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Pawn::new(false)), Box::new(Bishop::new(false))],
                [Box::new(Queen::new(true)),  Box::new(Pawn::new(true)), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Pawn::new(false)), Box::new(Queen::new(false))],
                [Box::new(King::new(true)),   Box::new(Pawn::new(true)), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Pawn::new(false)), Box::new(King::new(false))],
                [Box::new(Bishop::new(true)), Box::new(Pawn::new(true)), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Pawn::new(false)), Box::new(Bishop::new(false))],
                [Box::new(Knight::new(true)), Box::new(Pawn::new(true)), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Pawn::new(false)), Box::new(Knight::new(false))],
                [Box::new(Rook::new(true)),   Box::new(Pawn::new(true)), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Air::new()), Box::new(Pawn::new(false)), Box::new(Rook::new(false))],
            ],

            white_king: Pos::new(4, 0),
            black_king: Pos::new(4, 7),

            black_col: Color::new(0, 0, 0),
            white_col: Color::new(255, 255, 255),

            black_tile_col: Color::new(40, 40, 40),
            white_tile_col: Color::new(100, 100, 100),
        }
    }
    pub fn display(&self, white:bool)
    {
        let mut file:i32 = 7;
        let mut tile = true;

        while file >= 0
        {

            cons::set_fg(&self.white_tile_col);
            print!("{} ",file+1);
            cons::reset();

            for rank in 0..8
            {
                cons::do_char(
                    // tile color
                    if tile
                    { &self.white_tile_col }
                    else
                    { &self.black_tile_col },

                    // pice color
                    if self.grid[rank][file as usize].is_white()
                    { &self.white_col }
                    else
                    { &self.black_col },

                    self.grid[rank][file as usize].get_identity()
                );

                // flip tile
                tile = !tile;
            }

            print!("\n");
            file -= 1;
            tile = !tile;
        }

        print!("  ");
        cons::set_fg(&self.white_tile_col);

        for rank in 0..8
        {
            let ranks = ['a','b','c','d','e','f','g','h'];
            print!("{}",ranks[rank]);
        }
        cons::reset();
    }
}