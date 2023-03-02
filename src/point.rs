use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    #[must_use]
    pub fn from(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
