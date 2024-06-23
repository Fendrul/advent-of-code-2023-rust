use PerimeterDirection::{Down, Up};
use crate::entities::game_cell::GameCell;
use crate::entities::game_cell::GameCellType::{Animal, BL, BR, Empty, EmptyAnimal, Horizontal, TL, TR, Vertical};

pub struct PerimeterState {
    pub(crate) perimeter_direction: Option<PerimeterDirection>,
    pub(crate) is_inside_perimeter: bool,
}

impl PerimeterState {
    pub fn new() -> PerimeterState {
        PerimeterState {
            perimeter_direction: None,
            is_inside_perimeter: false,
        }
    }
    
    pub fn toggle_inside_perimeter(&mut self) {
        self.is_inside_perimeter = !self.is_inside_perimeter;
    }

    pub fn determine_perimeter(&mut self, cell: &GameCell) {
        if let Animal(animal_type) = cell {
            Self::determine_perimeter(self, animal_type);
            return;
        }

        match &self.perimeter_direction {
            None => {
                match cell {
                    Vertical => self.toggle_inside_perimeter(),
                    TL => self.perimeter_direction = Some(Down),
                    BL => self.perimeter_direction = Some(Up),
                    _ => unreachable!("Invalid cell type when no direction is set")
                }
            }

            Some(direction) => {
                match (direction, cell) {
                    (Down, BR) | (Up, TR) => self.toggle_inside_perimeter(),
                    _ => {}
                }

                match cell {
                    TR | BR => self.perimeter_direction = None,
                    Horizontal => {}
                    TL | BL | Vertical | Empty | EmptyAnimal | Animal(_) => unreachable!("Invalid cell type when a direction for the perimeter is set at cell {}", cell),
                }
            }
        }
    }
}

pub enum PerimeterDirection {
    Up,
    Down,
}