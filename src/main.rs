/* Chess Command!
*The classic game of Chess recreated in a console based application.
*
*Patrick Torgerson
*Austin Arrell
*2020
*/

mod piece;
mod util;

mod pawn;
mod rook;
mod knight;
mod bishop;
mod king;
mod queen;

use pawn::Pawn;
use rook::Rook;
use knight::Knight;
use bishop::Bishop;
use king::King;
use queen::Queen;

use piece::Piece;
use util::Pos;

/*
File: A-H Queen - King
Rank: 1-8 White - Black
*/

fn main()
{
    println!("\n____________________\n");

    //Instanciation for testing 
    let n = Knight::new(true, 'N');
    let b = Bishop::new(true, 'B');
    let r = Rook::new(true, 'R');
    let p = Pawn::new(true, ' ');
    let k = King::new(true, 'K');
    let q = Queen::new(true, 'Q');


    //Testing the verify_move functions
    let pos1 = Pos::new(4,5);
    let pos2 = Pos::new(6,7);
    println!("Rook result: {}",r.verify_move(&pos1,&pos2));
    println!("Bishop result: {}",b.verify_move(&pos1,&pos2));
    println!("King result: {}",k.verify_move(&pos1,&pos2));
    println!("Queen result: {}",q.verify_move(&pos1,&pos2));
    println!("Knight result: {}",n.verify_move(&pos1,&pos2));


    println!("\n____________________\n");
}


