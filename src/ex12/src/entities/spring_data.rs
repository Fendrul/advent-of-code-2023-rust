use crate::entities::field::Field;
use crate::entities::record::Record;

#[derive(Debug, Clone)]
pub struct SpringData {
    field: Field,
    record: Record
}