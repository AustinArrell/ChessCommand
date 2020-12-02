
use crate::util::Pos;
use crate::util;
use crate::board::Board;

pub struct Game
{
    running:bool,
    white_turn: bool,
    is_server: bool,
    board: Board

}

impl Game
{
    pub fn new(white_turn: bool, is_server: bool )-> Self
    {
        let board = Board::new(is_server);
        return Game{running:true,white_turn,is_server,board};
    }

    pub fn run(&self)
    {
        self.board.display();
        while self.running
        {
            let (piece,dest_string) = Game::parse_move(util::prompt("\nEnter move:" ));
            let dest_pos = Pos::from(&dest_string);
            if !dest_pos.valid()

            { println!("That move doesn't exsist!"); continue;}
            self.board.move_piece(self.white_turn,piece,dest_pos);
            //self.white_turn = !self.white_turn;  after we get move_piece finished we will pass turn back and forth.
        }
    }

    
    fn parse_move(input: String) -> (char, String) 
    {
        //Takes a string and parses the move data from it
        let piece = Game::get_piece(&input);
        let destination = Game::get_destination(&input);
        return (piece,destination);
    }

    fn get_piece(input: &String) -> char
    {
        //Parse the piece data from user input
        if input.chars().next().unwrap().is_uppercase()
        {return input.chars().next().unwrap() }
        else
        {return 'P' };

    }

    fn get_destination(input: &String) -> String
    {
        //Parse the destination data from user input
        if input.chars().next().unwrap().is_uppercase()
        { return input[1..].to_string() }
        else
        { return input.to_string() };
    }

}