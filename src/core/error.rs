use std::{fmt::Display, ops::Range};

#[derive(Debug)]
pub enum Error {
    IO(String),
    OutOfBounds {
        value: i32,
        range: Range<i32>
    },
    OutOfBounds2D {
        point: (f32, f32),
        range: (Range<f32>, Range<f32>),
    },
}

impl Error {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::IO(..) => "I/O",
            Self::OutOfBounds2D { .. } => "OutOfBounds2D",
            Self::OutOfBounds { .. } => "OutOfBounds",
        }
    }

    pub fn message(&self) -> Option<String> {
        match self {
            Self::IO(m) => Some(m.clone()),
            Self::OutOfBounds { value, range } => Some(format!("value {} out of bounds ({}..{})", value, range.start, range.end)),
            Self::OutOfBounds2D { point, range } => Some(format!(
                "point ({}, {}) is out of bounds ({}..{}, {}..{})",
                point.0, point.1, range.0.start, range.0.end, range.1.start, range.1.end
            )),
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.kind(),
            match self.message() {
                Some(m) => format!(": {}", m),
                None => String::new(),
            }
        )
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
