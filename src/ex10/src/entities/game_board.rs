use std::collections::HashSet;
use std::fmt::Debug;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use Direction::{Down, Left, Right};
use GameCellType::{BL, BR, Empty, Horizontal, TL, TR, Vertical};

use crate::entities::game_cell::{Coordinates, GameCell, GameCellType};
use crate::entities::game_cell::GameCellType::Animal;
use crate::entities::ghost::{Direction, Ghost};
use crate::entities::ghost::Direction::Up;
use crate::entities::table_vector::TableVector;

pub struct GameBoard {
    cells: Vec<Vec<GameCell>>,
    animal_coordinates: Coordinates,
}

impl GameBoard {
    pub fn new(cells: Vec<Vec<GameCell>>) -> GameBoard {
        let mut animal_coordinates = None;

        for (row, line) in cells.iter().enumerate() {
            for (col, cell) in line.iter().enumerate() {
                let row = isize::try_from(row).expect("Row is too big");
                let col = isize::try_from(col).expect("Col is too big");

                if let Animal = cell {
                    animal_coordinates = Some(Coordinates { row, col });
                }
            }
        }

        GameBoard {
            cells,
            animal_coordinates: animal_coordinates.expect("No animal cell found"),
        }
    }

    pub fn get_answers(&self) -> usize {
        let perimeter_size = self.get_perimeter();
        let first_answer = perimeter_size / 2 + usize::from(perimeter_size % 2 != 0);

        first_answer
    }
    fn get_perimeter(&self) -> usize {
        let ghosts = self.create_ghosts();
        let ghost = self.explore_board(ghosts).expect("Unable to find animal cell");

        let _perimeter: HashSet<GameCell> = ghost.get_path().iter()
            .filter_map(|coordinates| self.get_cell_from_coordinates(coordinates))
            .collect();

        for cell in &self.cells[0] {
            
        }

        ghost.get_steps()
    }

    fn explore_board(&self, ghosts: Vec<Ghost>) -> Option<Ghost> {
        ghosts.into_par_iter()
            .find_map_first(|ghost| {
                let mut current_ghost = ghost;
                while let Some(new_ghost) = self.move_ghost(current_ghost) {
                    if let Animal = self.get_cell_from_coordinates(new_ghost.get_coordinates())
                        .expect("Couldn't extract cell from the new ghost")
                    {
                        return Some(new_ghost);
                    }
                    current_ghost = new_ghost;
                }
                None
            })
    }

    fn move_ghost(&self, ghost: Ghost) -> Option<Ghost> {
        match self.get_cell_from_coordinates(ghost.get_coordinates()) {
            Some(cell) => {
                match (ghost.get_direction(), cell) {
                    (Right, BR) | (Left, BL) | (Up, Vertical) => Some(ghost.move_to(Up)),
                    (Right, TR) | (Left, TL) | (Down, Vertical) => Some(ghost.move_to(Down)),
                    (Left, Horizontal) | (Down, BR) | (Up, TR) => Some(ghost.move_to(Left)),
                    (Right, Horizontal) | (Down, BL) | (Up, TL) => Some(ghost.move_to(Right)),
                    _ => None
                }
            }
            None => None
        }
    }

    fn create_ghosts(&self) -> Vec<Ghost> {
        let mut ghosts = Vec::new();

        for (direction, valid_cells) in &[
            (Up, vec![Vertical, TR, TL]),
            (Down, vec![Vertical, BR, BL]),
            (Left, vec![Horizontal, TL, BL]),
            (Right, vec![Horizontal, TR, BR]),
        ] {
            let move_coordinates = self.animal_coordinates.move_to(*direction);
            if let Some(game_cell) = self.get_cell_from_coordinates(&move_coordinates) {
                if valid_cells.contains(&game_cell) {
                    ghosts.push(Ghost::new(move_coordinates, *direction));
                }
            }
        }

        ghosts
    }

    fn get_cell_from_coordinates(&self, coordinates: &Coordinates) -> Option<GameCell> {
        self.cells.table_get(coordinates.row, coordinates.col).copied()
    }
}

impl Debug for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.cells {
            for cell in line {
                let _ = match cell {
                    Empty => write!(f, "."),
                    TR => write!(f, "F"),
                    TL => write!(f, "7"),
                    BR => write!(f, "L"),
                    BL => write!(f, "J"),
                    Horizontal => write!(f, "|"),
                    Vertical => write!(f, "-"),
                    Animal => write!(f, "S")
                };
            }
            let _ = writeln!(f);
        }

        write!(f, "Rows: {}, Cols: {}", self.cells.rows(), self.cells.cols())
    }
}