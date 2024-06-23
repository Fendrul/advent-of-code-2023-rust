use std::collections::HashSet;
use std::fmt::Debug;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use Direction::{Down, Left, Right};
use GameCellType::{BL, BR, Empty, EmptyAnimal, Horizontal, TL, TR, Vertical};

use crate::entities::game_cell::{Coordinates, GameCell, GameCellType};
use crate::entities::game_cell::GameCellType::Animal;
use crate::entities::ghost::{Direction, Ghost};
use crate::entities::ghost::Direction::Up;
use crate::entities::perimeter_state;
use crate::entities::perimeter_state::{PerimeterDirection, PerimeterState};
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
                
                match cell {
                    EmptyAnimal | Animal(_) => { animal_coordinates = Some(Coordinates { row, col });}
                    _ => {}
                }
            }
        }

        GameBoard {
            cells,
            animal_coordinates: animal_coordinates.expect("No animal cell found"),
        }
    }

    pub fn get_answers(&mut self) -> usize {
        let perimeter_size = self.get_perimeter();
        perimeter_size / 2 + usize::from(perimeter_size % 2 != 0)
    }
    fn get_perimeter(&mut self) -> usize {
        let ghosts = self.create_ghosts();

        self.determine_animal_type(&ghosts);

        let ghost = self.explore_board(ghosts).expect("Unable to find animal cell");

        let mut perimeter: HashSet<&Coordinates> = ghost.get_path()
            .iter()
            .collect();
        perimeter.insert(&self.animal_coordinates);

        let mut cells_inside_perimeter = 0;
        for (row, cell_line) in self.cells.iter().enumerate() {
            let mut perimeter_state = PerimeterState::new();
            
            let row = isize::try_from(row).expect("Row is too big");

            for (col, cell) in cell_line.iter().enumerate() {
                let col = isize::try_from(col).expect("Col is too big");
                
                if perimeter.contains(&Coordinates {row, col} ) {
                    perimeter_state.determine_perimeter(cell);
                } else if perimeter_state.is_inside_perimeter {
                    cells_inside_perimeter += 1;
                }
            }
        }
        
        println!("Number of inner tiles: {}", cells_inside_perimeter);

        ghost.get_steps()
    }

    fn determine_perimeter(perimeter_state: &mut PerimeterState, cell: &GameCell) {
        if let Animal(animal_type) = cell {
            Self::determine_perimeter(perimeter_state, animal_type);
            return;
        }
        
        match &perimeter_state.perimeter_direction {
            None => {
                match cell {
                    Vertical => perimeter_state.toggle_inside_perimeter(),
                    TL => perimeter_state.perimeter_direction = Some(PerimeterDirection::Down),
                    BL => perimeter_state.perimeter_direction = Some(PerimeterDirection::Up),     
                    _ => unreachable!("Invalid cell type when no direction is set")
                }
            }
            
            Some(direction) => {
                match (direction, cell) {
                    (PerimeterDirection::Down, BR) | (PerimeterDirection::Up, TR) => perimeter_state.toggle_inside_perimeter(),
                    _ => {}
                }
                
                match cell {
                    TR | BR => perimeter_state.perimeter_direction = None,
                    Horizontal => {}
                    TL | BL | Vertical | Empty | EmptyAnimal | Animal(_) => unreachable!("Invalid cell type when a direction for the perimeter is set at cell {}", cell),
                }
            }
        }
    }

    fn determine_animal_type(&mut self, ghosts: &[Ghost]) {
        let mut animal_cell = self.get_mut_cell_from_coordinates(self.animal_coordinates)
            .expect("Couldn't extract cell from animal cell coordinates");

        let mut directions: Vec<&Direction>  = ghosts.iter()
            .map(Ghost::get_direction)
            .collect();

        directions.sort();
        
        let underneath_animal_type = match directions.as_slice() {
            [Up, Right] => BL,
            [Up, Left] => BR,
            [Up, Down] => Vertical,
            [Left, Right] => Horizontal,
            [Down, Right] => TL,
            [Down, Left] => TR,
            _ => panic!("Invalid directions when trying to determine animal type")
        };
        
        *animal_cell = Animal(Box::new(underneath_animal_type));
    }

    fn explore_board(&self, ghosts: Vec<Ghost>) -> Option<Ghost> {
        ghosts.into_par_iter()
            .find_map_first(|ghost| {
                let mut current_ghost = ghost;
                
                while let Some(new_ghost) = self.move_ghost(current_ghost) {
                    
                    if let Animal(_) = self.get_cell_from_coordinates(new_ghost.get_coordinates())
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
                if valid_cells.contains(game_cell) {
                    ghosts.push(Ghost::new(move_coordinates, *direction));
                }
            }
        }

        ghosts
    }

    fn get_cell_from_coordinates(&self, coordinates: &Coordinates) -> Option<&GameCell> {
        self.cells.table_get(coordinates.row, coordinates.col)
    }

    fn get_mut_cell_from_coordinates(&mut self, coordinates: Coordinates) -> Option<&mut GameCell> {
        self.cells.table_get_mut(coordinates.row, coordinates.col)
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
                    Animal(_) | EmptyAnimal => write!(f, "S")
                };
            }
            let _ = writeln!(f);
        }

        write!(f, "Rows: {}, Cols: {}", self.cells.rows(), self.cells.cols())
    }
}