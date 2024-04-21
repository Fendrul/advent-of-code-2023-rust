#[derive(Debug, Clone, Copy)]
pub struct Galaxy {
    pub y: i64,
    pub x: i64,
}

impl Galaxy {
    pub fn new(x: i64, y: i64) -> Galaxy {
        Galaxy { x, y }
    }
    
    pub fn calculate_distance(&self, other: &Galaxy) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
    
    pub fn add_to_x(&mut self, value: i64) {
        self.x += value;
    }
    
    pub fn add_to_y(&mut self, value: i64) {
        self.y += value;
    }
}