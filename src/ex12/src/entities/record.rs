#[derive(Debug, Clone)]
pub struct Record {
    map: Vec<i32>
}

impl Record {
    pub fn new(map: Vec<i32>) -> Record {
        Record {
            map: map.clone()
        }
    }
}