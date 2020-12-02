use crate::piece::Piece;

use crate::pawn::Pawn;
use crate::rook::Rook;
use crate::knight::Knight;
use crate::bishop::Bishop;
use crate::king::King;
use crate::queen::Queen;
use crate::air::Air;

use crate::util::{Pos,cons,cons::Color};
use crate::util;

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

    is_server: bool,
}

impl Board
{
    pub fn new(is_server:bool) -> Self
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
            is_server,
        }
    }
    pub fn display(&self)
    {
        //adjust which color will be displayed closer to the user
        let mut file:i32 = if self.is_server {7} else {0};  
        let end = if self.is_server {-1} else {8};

        let mut white_tile = true;

        while file != end
        {

            // file specifier
            cons::set_fg(&self.white_tile_col);
            print!("{} ",file+1);
            cons::reset();

            for rank in 0..8
            {
                cons::do_char(
                    // tile color
                    if white_tile
                    { &self.white_tile_col }
                    else
                    { &self.black_tile_col },

                    // piece color
                    if self.grid[rank][file as usize].is_white()
                    { &self.white_col }
                    else
                    { &self.black_col },

                    self.grid[rank][file as usize].get_symbol()
                );

                white_tile = !white_tile;
            }

            print!("\n");
            file += if self.is_server {-1} else {1};
            white_tile = !white_tile;
        }

        // rank specifiers

        print!("  ");
        cons::set_fg(&self.white_tile_col);

        
        for rank in 0..8
        {
            let ranks = ['a','b','c','d','e','f','g','h'];
            print!("{}",ranks[rank]);
        }
        cons::reset();
    }

    pub fn move_piece(&self,white_turn:bool,piece_identity:char,dest_pos:Pos)
    {
        println!("Trying to move:{} to:{},{}",piece_identity,dest_pos.rank+1,dest_pos.file+1);
        let assumed_piece = self.find_possible_move_candidate(white_turn,piece_identity,&dest_pos);

        if assumed_piece.rank > -1{
            let rank = assumed_piece.rank as usize;
            let file = assumed_piece.file as usize;
            if !self.grid[rank][file].verify_move(&assumed_piece,&dest_pos)
            {println!("That move is invalid!");}else
            {
                println!("Yeah, That could work.");
            }

            //TODO:
            //-check for collisions in path
            //-check for captures
            //update piece location
            
        }

    
        self.display();    
    }

    fn find_possible_move_candidate(&self,white_turn:bool,piece_identity:char,dest_pos:&Pos) -> Pos
    {
        let mut pos_of_piece = Pos::new(-1,-1);
        let mut num_of_candidates = 0;
        for file in 0..8
        {
            for rank in 0..8
            {
                //If the piece is the color of the current active player and that piece is not Air
                if self.grid[rank][file].is_white() == white_turn && self.grid[rank][file].get_identity() != ' '
                {
                    //If the piece identity matches the given identity
                    if self.grid[rank][file].get_identity() == piece_identity
                    {
                        //If this piece could move to that location
                        if self.grid[rank][file].verify_move(&Pos::new(rank as i32,file as i32),dest_pos)
                        {
                            //return the position of that piece
                            pos_of_piece.rank = rank as i32;
                            pos_of_piece.file = file as i32;
                            num_of_candidates += 1;

                        }
                    }
                }
            }
        }
        println!("Found :{} , possible candidates!",num_of_candidates);
        if num_of_candidates > 1
        {
            println!("Found multiple possible pieces for that move...");
            let input = util::prompt("\nSpecify start location of piece to move:" );
            pos_of_piece = Pos::from(&input);
        }
        return pos_of_piece;
    }


}

