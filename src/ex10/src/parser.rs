use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use GameCellType::Empty;
use ParsingError::InvalidData;
use Rule::{BL, BR, empty, TL, TR};

use crate::entities::game_board::GameBoard;
use crate::entities::game_cell::{GameCell, GameCellType};
use crate::parser::ParsingError::InvalidFileSyntax;

#[derive(Parser)]
#[grammar = "syntax.pest"]
pub struct AOCParser();

pub fn parse_stdin(input: &str) -> Result<GameBoard, ParsingError> {
    let parsed_game = match AOCParser::parse(Rule::game, input) {
        Ok(data) => data,
        Err(error) => {
            return Err(InvalidFileSyntax(error.to_string()));
        }
    }.next().unwrap();

    let mut game_board= Vec::new();

    for line in parsed_game.into_inner() {
        game_board.push(parse_line(line)?);
    }

    Ok(GameBoard::new(game_board))
}

fn parse_line(line: Pair<Rule>) -> Result<Vec<GameCell>, ParsingError> {
    let mut game_line= Vec::new();

    for cell in line.into_inner() {
        game_line.push(parse_cell(cell)?);
    }

    Ok(game_line)
}

fn parse_cell(cell: Pair<Rule>) -> Result<GameCell, ParsingError> {
    match cell.as_rule() {
        empty => Ok(Empty),
        TL => Ok(GameCellType::TL),
        TR => Ok(GameCellType::TR),
        BL => Ok(GameCellType::BL),
        BR => Ok(GameCellType::BR),
        Rule::horizontal => Ok(GameCellType::Horizontal),
        Rule::vertical => Ok(GameCellType::Vertical),
        Rule::animal => Ok(GameCellType::EmptyAnimal),
        _ => Err(InvalidData("Expected a cell type, couldn't find one"))
    }
}

pub enum ParsingError<'a> {
    InvalidFileSyntax(String),
    InvalidData(&'a str),
}