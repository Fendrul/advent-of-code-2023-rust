#[derive(Debug, Clone, Copy)]
pub enum FieldType {
    Unknown(i32),
    Grass(i32),
    AdjacentSprings(i32),
}

impl FieldType {
    pub const fn char(&self) -> char {
        match self {
            FieldType::Unknown(_) => '?',
            FieldType::Grass(_) => '.',
            FieldType::AdjacentSprings(_) => '#',
        }
    }
    
    pub const fn convert_to_field_type(character: &str, count: i32) -> FieldType {
        match character {
            "?" => FieldType::Unknown(count),
            "." => FieldType::Grass(count),
            "#" => FieldType::AdjacentSprings(count),
            _ => panic!("Invalid character"),
        }
    }
}