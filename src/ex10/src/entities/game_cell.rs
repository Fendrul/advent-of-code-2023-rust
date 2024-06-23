use std::fmt::{Debug, Display};
use std::hash::Hash;

use Direction::{Down, Left, Right, Up};
use crate::entities;

use crate::entities::ghost::Direction;

pub type GameCell = GameCellType;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum GameCellType {
    Empty,
    TR,
    TL,
    BR,
    BL,
    Horizontal,
    Vertical,
    Animal(Option<Box<GameCellType>>),
}

impl Display for GameCellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameCellType::Empty => write!(f, "Empty"),
            GameCellType::TL => write!(f, "F"),
            GameCellType::TR => write!(f, "7"),
            GameCellType::BR => write!(f, "J"),
            GameCellType::BL => write!(f, "L"),
            GameCellType::Horizontal => write!(f, "-"),
            GameCellType::Vertical => write!(f, "|"),
            GameCellType::Animal(_) => write!(f, "S"),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Coordinates {
    pub(crate) row: isize,
    pub(crate) col: isize,
}

impl Coordinates {
    pub fn move_to(&self, direction: Direction) -> Coordinates {
        match direction {
            Up => Coordinates { row: self.row - 1, col: self.col },
            Down => Coordinates { row: self.row + 1, col: self.col },
            Left => Coordinates { row: self.row, col: self.col - 1 },
            Right => Coordinates { row: self.row, col: self.col + 1 },
        }
    }
}