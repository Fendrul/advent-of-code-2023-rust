use crate::entities::game_cell::Coordinates;

#[derive(Clone)]
pub struct Ghost {
    coordinates: Coordinates,
    direction: Direction,
    path: Vec<Coordinates>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Ghost {
    pub fn new(coordinates: Coordinates, direction: Direction) -> Self {
        Ghost {
            coordinates,
            direction,
            path: Vec::new(),
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

    pub fn get_direction(&self) -> Direction {
        self.direction
    }
    
    pub fn get_path(&self) -> &Vec<Coordinates> {
        &self.path
    }

    pub fn get_steps(&self) -> usize {
        self.path.len()
    }
}
