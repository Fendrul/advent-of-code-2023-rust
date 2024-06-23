use crate::entities::game_cell::{Coordinates, GameCell};
use crate::entities::game_cell::GameCellType::{Animal, BL, BR, Horizontal, TL, TR, Vertical};
use crate::entities::ghost::Direction::{Down, Left, Right, Up};
use crate::entities::table_vector::TableVector;

#[derive(Clone)]
pub struct Ghost<'a> {
    coordinates: Coordinates,
    direction: Direction,
    path: Vec<Coordinates>,
    cells_table: &'a Vec<Vec<GameCell>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<'a> Ghost<'a> {
    pub fn new(cells_table: &'a Vec<Vec<GameCell>>, coordinates: Coordinates, direction: Direction) -> Self {
        Ghost {
            coordinates,
            direction,
            path: Vec::new(),
            cells_table,
        }
    }

    pub fn explore_board(self) -> Option<Ghost<'a>> {
        let mut current_ghost = self;

        while let Some(new_ghost) = current_ghost.move_ghost_forward() {
            if let Animal(_) = new_ghost.cells_table.table_get_from_coordinates(new_ghost.get_coordinates())
                .expect("Couldn't extract cell from the new ghost")
            {
                return Some(new_ghost);
            }

            current_ghost = new_ghost;
        }

        None
    }

    pub fn move_ghost_forward(self) -> Option<Ghost<'a>> {
        match self.cells_table.table_get_from_coordinates(self.get_coordinates()) {
            Some(cell) => {
                match (self.get_direction(), cell) {
                    (Right, BR) | (Left, BL) | (Up, Vertical) => Some(self.move_to(Up)),
                    (Right, TR) | (Left, TL) | (Down, Vertical) => Some(self.move_to(Down)),
                    (Left, Horizontal) | (Down, BR) | (Up, TR) => Some(self.move_to(Left)),
                    (Right, Horizontal) | (Down, BL) | (Up, TL) => Some(self.move_to(Right)),
                    _ => None
                }
            }
            None => None
        }
    }

    pub fn move_to(mut self, direction: Direction) -> Self {
        self.path.push(self.coordinates);
        self.coordinates = self.coordinates.move_to(direction);
        self.direction = direction;

        self
    }

    pub fn get_coordinates(&self) -> &Coordinates {
        &self.coordinates
    }

    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn get_path(&self) -> &Vec<Coordinates> {
        &self.path
    }

    pub fn get_steps(&self) -> usize {
        self.path.len()
    }
}
