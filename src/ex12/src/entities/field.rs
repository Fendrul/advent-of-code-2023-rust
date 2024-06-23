use crate::entities::field_type::FieldType;

#[derive(Debug, Clone)]
pub struct Field {
    field: Vec<FieldType>
}

impl Field {
    pub fn new(field: Vec<FieldType>) -> Field {
        Field {
            field: field.clone()
        }
    }
}