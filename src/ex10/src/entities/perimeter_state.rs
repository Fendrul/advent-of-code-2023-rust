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
}

pub enum PerimeterDirection {
    Up,
    Down,
}