use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Scalar(ScalarType),
    Array(ArrayType),
}

#[derive(Debug, Eq, PartialEq, EnumString)]
pub enum ScalarType {
    Void,
    String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ArrayType {
    type_identifier: ScalarType,
    dimensions: Dimensions,
}

impl ArrayType {
    pub fn new(type_identifier: ScalarType, dimensions: Dimensions) -> Self {
        ArrayType {
            type_identifier,
            dimensions,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Dimensions {
    start: u32,
    end: u32,
}

impl Dimensions {
    pub fn new(start: u32, end: u32) -> Self {
        Dimensions { start, end }
    }

    pub fn empty() -> Self {
        Self::new(0, 0)
    }

    pub fn start(&self) -> u32 {
        self.start
    }
    pub fn end(&self) -> u32 {
        self.end
    }
}

impl FromStr for Dimensions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "[]" {
            Ok(Self::new(0, 0))
        } else {
            Err(())
        }
    }
}
