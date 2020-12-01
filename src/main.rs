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

use std::io;
use std::io::Write;

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

    //Testing the verify_move functions
    let pos1 = Pos::new(4,5);
    let pos2 = Pos::new(6,7);
    println!("Rook result: {}",r.verify_move(&pos1,&pos2));
    println!("Bishop result: {}",b.verify_move(&pos1,&pos2));
    println!("King result: {}",k.verify_move(&pos1,&pos2));
    println!("Queen result: {}",q.verify_move(&pos1,&pos2));
    println!("Knight result: {}",n.verify_move(&pos1,&pos2));

    let running = true;
    while running
    {
        let mut input = String::new();

        print!("Enter starting square: ");

        // ensure 'Enter move: ' is printed before we pause for user input
        io::stdout().flush().expect("Console write error");

        io::stdin().read_line(&mut input).expect("Could not read line!");

        let start_pos = Pos::from(&input);

        if !start_pos.valid()
        { continue; }

        print!("Enter move: ");

        // ensure 'Enter move: ' is printed before we pause for user input
        io::stdout().flush().expect("Console write error");

        io::stdin().read_line(&mut input).expect("Could not read line!");

        let piece =
        if input.chars().next().unwrap().is_uppercase()
        { input.chars().next().unwrap() }
        else
        { 'P' };

        let destination =
        if input.chars().next().unwrap().is_uppercase()
        { input[1..].to_string() }
        else
        { input };

        let dest_pos = Pos::from(&destination);

        if !dest_pos.valid()
        { continue; }

        match piece
        {
            'Q' => println!("{}", q.verify_move(&start_pos, &dest_pos)),
            'K' => println!("{}", k.verify_move(&start_pos, &dest_pos)),
            'R' => println!("{}", r.verify_move(&start_pos, &dest_pos)),
            'B' => println!("{}", b.verify_move(&start_pos, &dest_pos)),
            'N' => println!("{}", n.verify_move(&start_pos, &dest_pos)),
            'P' => println!("{}", p.verify_move(&start_pos, &dest_pos)),
            _   => println!("invalid piece!")
        }
    }

    println!("\n____________________\n");
}