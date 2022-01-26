use std::ops::Range;

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Scalar(ScalarType),
    Array(ArrayType),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ScalarType {
    Void,
    String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ArrayType {
    type_identifier: ScalarType,
    dimensions: Dimensions,
}

pub type Dimensions = Range<u32>;

impl ArrayType {
    pub fn new(type_identifier: ScalarType, dimensions: Dimensions) -> Self {
        ArrayType {
            type_identifier,
            dimensions,
        }
    }
}
