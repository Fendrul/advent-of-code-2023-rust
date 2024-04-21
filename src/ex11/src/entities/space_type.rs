use crate::entities::galaxy::Galaxy;

#[derive(Debug, Clone, Copy)]
pub enum SpaceType {
    EmptySpace,
    GalaxySpace(Galaxy),
}

impl SpaceType {
    pub const fn value(&self) -> char {
        match *self {
            SpaceType::EmptySpace => '.',
            SpaceType::GalaxySpace(_) => '#',
        }
    }
}