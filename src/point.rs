#[derive(Debug, Default, Clone, Copy)]
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
