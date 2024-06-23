use std::hash::Hash;

use Direction::{Down, Left, Right, Up};

use crate::entities::ghost::Direction;

pub type GameCell = GameCellType;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum GameCellType {
    Empty,
    TR,
    TL,
    BR,
    BL,
    Horizontal,
    Vertical,
    Animal,
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