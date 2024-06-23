use std::collections::HashSet;
use std::fmt::Display;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use Direction::{Down, Left, Right};
use GameCellType::{BL, BR, EmptyAnimal, Horizontal, TL, TR, Vertical};

use crate::entities::game_cell::{Coordinates, GameCell, GameCellType};
use crate::entities::game_cell::GameCellType::Animal;
use crate::entities::ghost::{Direction, Ghost};
use crate::entities::ghost::Direction::Up;
use crate::entities::perimeter_state::PerimeterState;
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
                let row: i32 = row.try_into().expect("Row is too big");
                let col: i32 = col.try_into().expect("Col is too big");

                if let EmptyAnimal = cell {
                    animal_coordinates = Some(Coordinates { row, col });
                }
            }
        }

        let mut game_board = GameBoard {
            cells,
            animal_coordinates: animal_coordinates.expect("No animal cell found"),
        };

        game_board.determine_animal_type();

        game_board
    }

    fn determine_animal_type(&mut self) {
        let mut directions = self.get_valid_directions_from_animal_coordinates();
        let animal_cell = self.get_mut_cell_from_coordinates(self.animal_coordinates)
            .expect("Couldn't extract cell from animal cell coordinates");


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

    pub fn get_answers(&mut self) -> (usize, i32) {
        let ghosts = self.create_ghosts();

        let ghost = get_successful_ghost(ghosts)
            .expect("Error while trying to explore the board with the ghosts");

        let farthest_point = calculate_farthest_point(ghost.get_steps());
        let cells_inside_perimeter = self.calculate_inner_tiles(ghost.get_path());

        (farthest_point, cells_inside_perimeter)
    }

    fn create_ghosts(&self) -> Vec<Ghost> {
        let mut ghosts = Vec::new();

        let directions = self.get_valid_directions_from_animal_coordinates();

        for direction in directions {
            let coordinates_from_direction = self.animal_coordinates.move_to(direction);

            if self.get_cell_from_coordinates(&coordinates_from_direction).is_some() {
                ghosts.push(Ghost::new(&self.cells, coordinates_from_direction, direction));
            }
        }

        ghosts
    }

    fn get_valid_directions_from_animal_coordinates(&self) -> Vec<Direction> {
        let mut valid_directions = Vec::new();

        for (direction, valid_cells) in &[
            (Up, vec![Vertical, TR, TL]),
            (Down, vec![Vertical, BR, BL]),
            (Left, vec![Horizontal, TL, BL]),
            (Right, vec![Horizontal, TR, BR]),
        ] {
            let move_coordinates = self.animal_coordinates.move_to(*direction);
            if let Some(cell) = self.get_cell_from_coordinates(&move_coordinates) {
                if valid_cells.contains(cell) {
                    valid_directions.push(*direction);
                }
            }
        }

        valid_directions
    }

    fn calculate_inner_tiles(&self, perimeter: &[Coordinates]) -> i32 {
        let perimeter = get_hashet_perimeter(&self.animal_coordinates, perimeter);

        let mut cells_inside_perimeter = 0;
        for (row, cell_line) in self.cells.iter().enumerate() {
            let mut perimeter_state = PerimeterState::new();

            for (col, cell) in cell_line.iter().enumerate() {
                let row: i32 = row.try_into().expect("Row is too big");
                let col: i32 = col.try_into().expect("Col is too big");

                if perimeter.contains(&Coordinates { row, col }) {
                    perimeter_state.determine_perimeter(cell);
                } else if perimeter_state.is_inside_perimeter {
                    cells_inside_perimeter += 1;
                }
            }
        }

        cells_inside_perimeter
    }

    fn get_cell_from_coordinates(&self, coordinates: &Coordinates) -> Option<&GameCell> {
        self.cells.table_get(coordinates.row, coordinates.col)
    }

    fn get_mut_cell_from_coordinates(&mut self, coordinates: Coordinates) -> Option<&mut GameCell> {
        self.cells.table_get_mut(coordinates.row, coordinates.col)
    }
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.cells {
            for cell in line {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }

        write!(f, "Rows: {}, Cols: {}", self.cells.rows(), self.cells.cols())
    }
}

fn get_successful_ghost(ghosts: Vec<Ghost>) -> Option<Ghost> {
    ghosts.into_par_iter()
        .find_map_first(|ghost| {
            ghost.explore_board()
        })
}

fn calculate_farthest_point(ghost_steps: usize) -> usize {
    ghost_steps / 2 + usize::from(ghost_steps % 2 != 0)
}

fn get_hashet_perimeter<'a>(animal_coordinates: &'a Coordinates, perimeter: &'a [Coordinates]) -> HashSet<&'a Coordinates> {
    let mut perimeter: HashSet<&Coordinates> = perimeter
        .iter()
        .collect();
    perimeter.insert(animal_coordinates);

    perimeter
}
