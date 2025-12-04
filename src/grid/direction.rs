use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,

    UpRight,
    UpLeft,

    DownRight,
    DownLeft,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "UP"),
            Direction::Down => write!(f, "DOWN"),
            Direction::Left => write!(f, "LEFT"),
            Direction::Right => write!(f, "RIGHT"),
            Direction::UpRight => write!(f, "UP-RIGHT"),
            Direction::UpLeft => write!(f, "UP-LEFT"),
            Direction::DownRight => write!(f, "DOWN-RIGHT"),
            Direction::DownLeft => write!(f, "DOWN-LEFT"),
        }
    }
}

impl Direction {
    pub fn items() -> [Direction; 8] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ]
    }
}
