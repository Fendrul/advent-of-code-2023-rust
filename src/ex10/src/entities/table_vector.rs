use crate::entities::game_cell::Coordinates;

pub trait TableVector<T> {
    fn table_get(&self, row: impl TryInto<usize>, col: impl TryInto<usize>) -> Option<&T>;
    fn table_get_mut(&mut self, row: impl TryInto<usize>, col: impl TryInto<usize>) -> Option<&mut T>;
    fn table_get_from_coordinates(&self, coordinates: &Coordinates) -> Option<&T>;
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
}

impl<T> TableVector<T> for Vec<Vec<T>> {
    fn table_get(&self, row: impl TryInto<usize>, col: impl TryInto<usize>) -> Option<&T> {
        let row = row.try_into().ok()?;
        let col = col.try_into().ok()?;
        
        self.get(row).and_then(|r| r.get(col))
    }

    fn table_get_mut(&mut self, row: impl TryInto<usize>, col: impl TryInto<usize>) -> Option<&mut T> {
        let row = row.try_into().ok()?;
        let col = col.try_into().ok()?;
        
        self.get_mut(row).and_then(|r| r.get_mut(col))
    }

    fn table_get_from_coordinates(&self, coordinates: &Coordinates) -> Option<&T> {
        self.table_get(coordinates.row, coordinates.col)
    }

    fn rows(&self) -> usize {
        self.len()
    }

    fn cols(&self) -> usize {
        self.first().map_or(0, Vec::len)
    }
}