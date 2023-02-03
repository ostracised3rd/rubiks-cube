
#[derive(Debug, Clone)]
pub enum Color {
    YELLOW = 1,
    PURPLE = 2,
    RED = 3,
    BLUE = 4,
    GREEN = 5,
    WHITE = 6,
}

#[derive(Debug, Clone)]
pub enum Face {
    UP = 1,
    FRONT = 2,
    LEFT = 3,
    BACK = 4,
    RIGHT = 5,
    DOWN = 6,
}

#[derive(Debug, Clone)]
pub enum Category {
    CORNER = 1,
    EDGE = 2,
    CENTER = 3,
    BASE = 4,
}