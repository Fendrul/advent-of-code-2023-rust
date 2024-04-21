use std::fmt::Display;

use crate::entities::space_type::SpaceType;
use crate::entities::space_type::SpaceType::{EmptySpace, GalaxySpace};

type Space = Vec<Vec<SpaceType>>;

#[derive(Debug, Clone)]
pub struct Cosmos {
    pub space: Vec<Vec<SpaceType>>,
    pub galaxies: Vec<SpaceType>,
}

impl Cosmos {
    pub fn new(space: Space) -> Cosmos {
        let mut galaxies: Vec<SpaceType> = Vec::new();

        for line in &space {
            for space in line {
                if let GalaxySpace(_galaxy) = space {
                    galaxies.push(*space)
                }
            }
        }

        Cosmos {
            space,
            galaxies,
        }
    }

    pub fn expend_cosmos(&self, expend_value: i64) -> Cosmos {
        let mut cosmos_to_expand = self.space.clone();

        expand_verticaly(&mut cosmos_to_expand, expend_value);
        expand_horizontaly(&mut cosmos_to_expand ,expend_value);

        Cosmos::new(cosmos_to_expand)
    }
}

impl Cosmos {
    pub fn calculate_space_between_galaxies(&self) -> i64 {
        let mut space_between_galaxies: i64 = 0;

        for (index, galaxy) in self.galaxies.iter().enumerate() {
            for other_galaxy in &self.galaxies[index + 1..] {
                if let (GalaxySpace(galaxy), GalaxySpace(other_galaxy)) = (galaxy, other_galaxy) {
                    space_between_galaxies += galaxy.calculate_distance(other_galaxy);
                }
            }
        }

        space_between_galaxies
    }
}

impl Display for Cosmos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.space {
            for space in line {
                write!(f, "{}", space.value())?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn expand_verticaly(cosmos_vec: &mut [Vec<SpaceType>], expend_value: i64) {
    let mut number_of_empty_lines: i64 = 0;

    cosmos_vec.iter_mut().for_each(|cosmos_line| {
        let mut is_empty = true;
        
        for space in &mut *cosmos_line {
            match space {
                GalaxySpace(galaxy) => {
                    is_empty = false;

                    galaxy.add_to_y(number_of_empty_lines * expend_value);
                }
                EmptySpace => {}
            }
        }

        if is_empty {
            number_of_empty_lines += 1;
        }
    })
}

fn expand_horizontaly(cosmos: &mut [Vec<SpaceType>], expend_value: i64) {
    let mut number_of_empty_columns: i64 = 0;

    for column_index in 0..cosmos[0].len() {
        let mut is_empty_line = true;

        for line in &mut *cosmos {
            match &mut line[column_index] {
                GalaxySpace(galaxy) => {
                    is_empty_line = false;

                    galaxy.add_to_x(number_of_empty_columns * expend_value);
                }
                EmptySpace => {}
            }
        }

        if is_empty_line {
            number_of_empty_columns += 1;
        }
    }
}