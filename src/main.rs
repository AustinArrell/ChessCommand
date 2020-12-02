/* Chess Command!
*The classic game of Chess recreated in a console based application.
*
*Patrick Torgerson
*Austin Arrell
*2020
*/

mod piece;
mod util;
mod game;
mod board;

mod air;
mod pawn;
mod rook;
mod knight;
mod bishop;
mod king;
mod queen;

use air::Air;
use pawn::Pawn;
use rook::Rook;
use knight::Knight;
use bishop::Bishop;
use king::King;
use queen::Queen;

use piece::Piece;
use util::Pos;
use game::Game;
use board::Board;


/*
Rank: A-H Queen - King
File: 1-8 White - Black
*/

fn main()
{
    println!("\n____________________\n");

    //Instanciation for testing
    let n = Knight::new(true);
    let b = Bishop::new(true);
    let r = Rook::new(true);
    let p = Pawn::new(true);
    let k = King::new(true);
    let q = Queen::new(true);

    let game = Game::new(true,true);
    game.run();



}
