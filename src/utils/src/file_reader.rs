use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

pub struct FileReader {
    reader: BufReader<File>,
}

impl FileReader {
    pub fn new(path_to_file_string: String) -> FileReader {
        let path_to_project: PathBuf = get_path_to_project();
        let path_to_file = path_to_project.join(path_to_file_string);

        let file = match File::open(path_to_file) {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        let buf_reader = BufReader::new(file.try_clone().unwrap());

        FileReader { reader: buf_reader }
    }

    pub fn read_line(&mut self) -> Option<String> {
        let mut line = String::new();

        match self.reader.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => Some(line),
            Err(_) => None,
        }
    }
}

impl Iterator for FileReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_line()
    }
}

fn get_path_to_project() -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    let mut new_path = PathBuf::new();

    for component in current_dir.components() {
        match component {
            std::path::Component::Prefix(prefix) => {
                new_path.push(prefix.as_os_str());
            }

            std::path::Component::RootDir => {
                new_path.push(std::path::MAIN_SEPARATOR.to_string());
            }

            std::path::Component::Normal(os_str) => {
                if os_str == "src" {
                    break;
                } else {
                    new_path.push(os_str);
                }
            }

            _ => {}
        }
    }

    new_path
}