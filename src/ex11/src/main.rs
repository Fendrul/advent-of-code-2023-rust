use std::time::Instant;

use utils::file_reader::FileReader;

use crate::entities::cosmos::Cosmos;
use crate::entities::galaxy::Galaxy;
use crate::entities::space_type::SpaceType;
use crate::entities::space_type::SpaceType::{EmptySpace, GalaxySpace};

pub mod entities;

fn main() {
    let start = Instant::now();

    let file_reader = FileReader::new(String::from("src/ex11/src/input/solution"));

    let input: Vec<Vec<char>> = file_reader.into_iter()
        .map(|line| {
            line.chars()
                .filter(|c| c == &'.' || c == &'#')
                .collect()
        }).collect();

    let base_cosmos = fill_cosmos(&input);
    
    let first_cosmos_expended = base_cosmos.expend_cosmos(1);
    println!("first solution : {}", first_cosmos_expended.calculate_space_between_galaxies());
    
    let second_cosmos_expended = base_cosmos.expend_cosmos(1000000 - 1);
    
    println!("second solution : {}", second_cosmos_expended.calculate_space_between_galaxies());

    println!("Time duration of the process: {:?}", start.elapsed());
}

pub fn fill_cosmos(input: &[Vec<char>]) -> Cosmos {
    let mut cosmos: Vec<Vec<SpaceType>> = Vec::new();

    for (line_count, line) in input.iter().enumerate() {
        let cosmos_line: Vec<SpaceType> = line.iter()
            .enumerate()
            .map(|(char_index, char)| match char {
                '.' => EmptySpace,
                '#' => GalaxySpace(Galaxy::new(char_index as i64, line_count as i64)),
                _ => panic!("Invalid character in input file")
            }).collect();

        cosmos.push(cosmos_line);
    }


    Cosmos::new(cosmos)
}

// fn expand_cosmos(cosmos: &[Vec<char>]) -> Vec<Vec<char>> {
//     let mut cosmos_to_expand = cosmos.to_owned();
// 
//     expand_horizontaly(&mut cosmos_to_expand);
//     expand_vertically(&mut cosmos_to_expand);
// 
//     cosmos_to_expand
// }
// 
// fn expand_vertically(cosmos: &mut Vec<Vec<char>>) {
//     let mut column_index_to_fill: Vec<usize> = Vec::new();
// 
//     for column_index in 0..cosmos[0].len() {
//         let mut is_empty = true;
// 
//         for line in &*cosmos {
//             if line[column_index] != EmptySpace.value() {
//                 is_empty = false;
//                 break;
//             }
//         }
// 
//         if is_empty {
//             column_index_to_fill.push(column_index);
//         }
//     }
// 
//     column_index_to_fill.iter().enumerate().for_each(|(number_of_column_added, column_index)| {
//         for line in &mut *cosmos {
//             line.insert(*column_index + number_of_column_added + 1, EmptySpace.value());
//         }
//     });
// }
// 
// fn expand_horizontaly(cosmos: &mut Vec<Vec<char>>) {
//     let mut line_index_to_fill: Vec<usize> = Vec::new();
// 
//     for (line_index, galaxy_line) in cosmos.iter().enumerate() {
//         let mut is_empty = true;
//         for char in galaxy_line {
// 
//             if char != &EmptySpace.value() {
//                 is_empty = false;
//                 break;
//             }
//         }
// 
//         if is_empty {
//             line_index_to_fill.push(line_index);
//         }
//     }
// 
//     line_index_to_fill.iter().enumerate().for_each(|(number_of_line_added, line_index)| {
//         let new_line = vec![EmptySpace.value(); cosmos[*line_index].len()];
// 
//         cosmos.insert(*line_index + number_of_line_added + 1, new_line);
//     });
// }
