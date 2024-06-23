use std::time::Instant;
use file_reader::file_reader::FileReader;
use parser::parse_stdin;
use ParsingError::{InvalidData, InvalidFileSyntax};

use crate::parser::ParsingError;

mod parser;
mod entities;

fn main() {
    let timer = Instant::now();

    let file_content = match FileReader::read_file(r"src\ex10\src\input") {
        Ok(content) => content,
        Err(error) => panic!("Error while tryign to read the file: {:?}", error)
    };

    let game_board = match parse_stdin(&file_content) {
        Ok(content) => content,
        Err(error) =>
            match error {
                InvalidFileSyntax => panic!("Invalid file syntax"),
                InvalidData(error_message) => panic!("Invalid data: {}", error_message)
            }
    };
    
    println!("Farthest point: {}", game_board.get_answers());
    
    // println!("{:?}", game_board);
    println!("Execution time: {:?}", timer.elapsed());
}