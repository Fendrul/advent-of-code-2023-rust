use std::net::TcpListener;
use regex::Regex;
use utils::file_reader::FileReader;
use crate::entities::field_type::FieldType;
use crate::entities::field_type::FieldType::{AdjacentSprings, Grass, Unknown};
use crate::entities::spring_data::SpringData;

pub fn build_from_file(file_reader: FileReader) -> SpringData {
    file_reader.for_each(|line| {
        println!("{}", line);
        parse_line(line);
    });
    
    TcpListener
    
}

fn parse_line(line: String) -> SpringData {
    let re = Regex::new(r"[?#.]").unwrap();
    let numbers_re = Regex::new(r"\d+").unwrap();

    let mut groups = Vec::new();
    let mut numbers = Vec::new();
    let mut last_char = None;
    let mut count = 0;

    for cap in re.captures_iter(&line) {
        let character = cap[0].to_string();

        if Some(character.clone()) == last_char {
            count += 1;
        } else {
            if let Some(last_char) = last_char {
                groups.push(FieldType::convert_to_field_type(&*last_char, count));
            }
            last_char = Some(character);
            count = 1;
        }
    }

    if let Some(last_char) = last_char {
        groups.push(FieldType::convert_to_field_type(&*last_char, count));
    }

    for cap in numbers_re.captures_iter(&line[line.find(" ").unwrap_or(0)..]) {
        let number = cap[0].parse::<i32>().unwrap();
        numbers.push(number);
    }

    // SpringData {
    //     groups,
    //     numbers,
    // }
    
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::file_reader::FileReader;

    #[test]
    fn test_build_from_file() {
        let file_reader = FileReader::new(String::from("src/ex12/src/inputs/test"));
        let spring_data = build_from_file(file_reader);
        assert_eq!(2, 2);
    }
    
    #[test]
    fn test_parse_line() {
        let line = String::from("????.######..#####. 1, 1, 3");
        
        
    }
}